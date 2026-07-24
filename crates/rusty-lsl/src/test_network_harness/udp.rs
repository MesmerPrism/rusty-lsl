//! Test-only ownership for a responder running on an already-bound UDP socket.

use std::any::Any;
use std::net::{SocketAddr, UdpSocket};
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

enum WorkerOutcome<T> {
    Completed(T),
    Panicked(Box<dyn Any + Send>),
}

/// One-shot proof that fixture setup has reached the responder call itself.
pub(crate) struct UdpResponderEntry(SyncSender<()>);

impl UdpResponderEntry {
    pub(crate) fn publish(self) {
        self.0
            .send(())
            .expect("UDP responder owner dropped before worker entry rendezvous");
    }
}

/// Owns one prebound socket, its cooperative cancellation, completion
/// rendezvous, and the worker join handle.
pub(crate) struct SpawnedUdpResponder<T> {
    local_address: SocketAddr,
    cancelled: Arc<AtomicBool>,
    completion: Receiver<WorkerOutcome<T>>,
    worker: Option<JoinHandle<()>>,
    cleanup_timeout: Duration,
}

impl<T: Send + 'static> SpawnedUdpResponder<T> {
    /// Transfers the prebound socket to a worker and does not return until the
    /// worker has reached its actual entry point.
    pub(crate) fn spawn<F>(socket: UdpSocket, entry_timeout: Duration, run: F) -> Self
    where
        F: FnOnce(UdpSocket, &AtomicBool, UdpResponderEntry) -> T + Send + 'static,
    {
        let local_address = socket
            .local_addr()
            .expect("prebound UDP responder socket must expose its local address");
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancelled = Arc::clone(&cancelled);
        let (entered_tx, entered_rx) = mpsc::sync_channel(0);
        let (completion_tx, completion) = mpsc::sync_channel(0);
        let worker = thread::Builder::new()
            .name("rusty-lsl-test-udp-responder".into())
            .spawn(move || {
                let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
                    run(
                        socket,
                        worker_cancelled.as_ref(),
                        UdpResponderEntry(entered_tx),
                    )
                }))
                .map(WorkerOutcome::Completed)
                .unwrap_or_else(WorkerOutcome::Panicked);
                let _ = completion_tx.send(outcome);
            })
            .expect("failed to spawn prebound UDP responder worker");

        match entered_rx.recv_timeout(entry_timeout) {
            Ok(()) => Self {
                local_address,
                cancelled,
                completion,
                worker: Some(worker),
                cleanup_timeout: entry_timeout,
            },
            Err(error) => {
                cancelled.store(true, Ordering::Release);
                drop(entered_rx);
                drop(completion);
                let join = worker.join();
                panic!(
                    "prebound UDP responder at {local_address} did not publish worker entry within {entry_timeout:?}: {error}; join={join:?}"
                );
            }
        }
    }

    /// Returns the address proven bound before the worker was spawned.
    pub(crate) const fn local_address(&self) -> SocketAddr {
        self.local_address
    }

    /// Waits for bounded completion, cancelling and joining before reporting a
    /// timeout so the socket cannot remain owned by a detached worker.
    pub(crate) fn complete(mut self, timeout: Duration) -> T {
        let outcome = match self.completion.recv_timeout(timeout) {
            Ok(outcome) => outcome,
            Err(RecvTimeoutError::Timeout) => {
                self.cancelled.store(true, Ordering::Release);
                match self.completion.recv_timeout(timeout) {
                    Ok(outcome) => outcome,
                    Err(error) => {
                        let address = self.local_address;
                        let join = self.take_and_join();
                        panic!(
                            "prebound UDP responder at {address} timed out after {timeout:?}; cancellation did not complete within another {timeout:?}: {error}; join={join:?}"
                        );
                    }
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                let address = self.local_address;
                let join = self.take_and_join();
                panic!(
                    "prebound UDP responder at {address} disconnected without completion evidence; join={join:?}"
                );
            }
        };
        let join = self.take_and_join();
        assert!(join.is_ok(), "UDP responder join failed after completion");
        match outcome {
            WorkerOutcome::Completed(value) => value,
            WorkerOutcome::Panicked(payload) => panic::resume_unwind(payload),
        }
    }

    fn take_and_join(&mut self) -> thread::Result<()> {
        self.worker
            .take()
            .expect("UDP responder join handle must be owned exactly once")
            .join()
    }
}

impl<T> Drop for SpawnedUdpResponder<T> {
    fn drop(&mut self) {
        let Some(worker) = self.worker.take() else {
            return;
        };
        self.cancelled.store(true, Ordering::Release);
        let completion = self.completion.recv_timeout(self.cleanup_timeout);
        let join = worker.join();
        if !thread::panicking() {
            assert!(
                completion.is_ok(),
                "prebound UDP responder at {} did not complete during drop cleanup within {:?}",
                self.local_address,
                self.cleanup_timeout
            );
            assert!(join.is_ok(), "UDP responder panicked during drop cleanup");
        }
    }
}
