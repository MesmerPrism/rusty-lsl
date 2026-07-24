//! TCP spawned-peer lifecycle owner.

use std::any::Any;
use std::fmt;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc::{sync_channel, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub(crate) struct SpawnedTcpPeer<T> {
    label: &'static str,
    endpoint: SocketAddr,
    completion: Receiver<T>,
    worker: Option<JoinHandle<()>>,
}

#[derive(Debug)]
pub(crate) enum SpawnedTcpPeerError {
    TimedOut {
        label: &'static str,
        endpoint: SocketAddr,
        timeout: Duration,
    },
    Panicked {
        label: &'static str,
        endpoint: SocketAddr,
        message: String,
    },
    CompletionDisconnected {
        label: &'static str,
        endpoint: SocketAddr,
    },
}

impl fmt::Display for SpawnedTcpPeerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimedOut {
                label,
                endpoint,
                timeout,
            } => write!(
                formatter,
                "TCP peer {label} at {endpoint} did not complete within {timeout:?}"
            ),
            Self::Panicked {
                label,
                endpoint,
                message,
            } => write!(
                formatter,
                "TCP peer {label} at {endpoint} panicked: {message}"
            ),
            Self::CompletionDisconnected { label, endpoint } => write!(
                formatter,
                "TCP peer {label} at {endpoint} exited without publishing completion"
            ),
        }
    }
}

impl<T: Send + 'static> SpawnedTcpPeer<T> {
    pub(crate) fn spawn(
        label: &'static str,
        listener: TcpListener,
        run: impl FnOnce(TcpListener) -> T + Send + 'static,
    ) -> Self {
        let endpoint = listener.local_addr().unwrap_or_else(|error| {
            panic!("TCP peer {label} listener has no local address: {error}")
        });
        let (entered_sender, entered_receiver) = sync_channel(0);
        let (completion_sender, completion) = sync_channel(0);
        let worker = thread::Builder::new()
            .name(format!("tcp-test-peer-{label}"))
            .spawn(move || {
                entered_sender.send(()).unwrap_or_else(|error| {
                    panic!("TCP peer {label} could not publish worker entry: {error}")
                });
                let result = run(listener);
                let _ = completion_sender.send(result);
            })
            .unwrap_or_else(|error| panic!("could not spawn TCP peer {label}: {error}"));
        entered_receiver.recv().unwrap_or_else(|error| {
            panic!("TCP peer {label} exited before publishing worker entry: {error}")
        });
        Self {
            label,
            endpoint,
            completion,
            worker: Some(worker),
        }
    }

    pub(crate) fn endpoint(&self) -> SocketAddr {
        self.endpoint
    }

    pub(crate) fn complete(mut self, timeout: Duration) -> Result<T, SpawnedTcpPeerError> {
        let completion = self.completion.recv_timeout(timeout);
        let joined = self.join_worker();
        if let Err(payload) = joined {
            return Err(SpawnedTcpPeerError::Panicked {
                label: self.label,
                endpoint: self.endpoint,
                message: panic_message(payload),
            });
        }
        match completion {
            Ok(value) => Ok(value),
            Err(RecvTimeoutError::Timeout) => Err(SpawnedTcpPeerError::TimedOut {
                label: self.label,
                endpoint: self.endpoint,
                timeout,
            }),
            Err(RecvTimeoutError::Disconnected) => {
                Err(SpawnedTcpPeerError::CompletionDisconnected {
                    label: self.label,
                    endpoint: self.endpoint,
                })
            }
        }
    }

    fn join_worker(&mut self) -> thread::Result<()> {
        self.worker
            .take()
            .expect("TCP peer worker already joined")
            .join()
    }
}

impl<T> Drop for SpawnedTcpPeer<T> {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            if !thread::panicking() {
                if let Err(payload) = worker.join() {
                    panic!(
                        "TCP peer {} at {} panicked during cleanup: {}",
                        self.label,
                        self.endpoint,
                        panic_message(payload)
                    );
                }
            } else {
                let _ = worker.join();
            }
        }
    }
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_owned(),
            Err(_) => "non-string panic payload".to_owned(),
        },
    }
}
