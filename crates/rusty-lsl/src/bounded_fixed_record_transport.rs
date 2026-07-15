// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crate-private bounded exact-length TCP record transfer.

use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Closed transport-stage failure mapped by each owning runtime facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BoundedFixedRecordError {
    /// Caller cancellation was observed.
    Cancelled,
    /// The caller's total deadline elapsed.
    Deadline,
    /// The peer closed before the exact record length arrived.
    Truncated {
        /// Bytes received before peer closure.
        actual: usize,
    },
    /// One socket operation failed.
    Io(ErrorKind),
}

/// Writes exactly one already encoded record under caller-owned bounds.
pub(crate) fn write_exact_bounded(
    stream: &mut TcpStream,
    record: &[u8],
    io_slice: Duration,
    total_deadline: Duration,
    cancelled: &AtomicBool,
) -> Result<(), BoundedFixedRecordError> {
    let started = Instant::now();
    let mut offset = 0;
    while offset < record.len() {
        if cancelled.load(Ordering::Acquire) {
            return Err(BoundedFixedRecordError::Cancelled);
        }
        let remaining = total_deadline
            .checked_sub(started.elapsed())
            .ok_or(BoundedFixedRecordError::Deadline)?;
        stream
            .set_write_timeout(Some(remaining.min(io_slice)))
            .map_err(|error| BoundedFixedRecordError::Io(error.kind()))?;
        match stream.write(&record[offset..]) {
            Ok(0) => return Err(BoundedFixedRecordError::Io(ErrorKind::WriteZero)),
            Ok(written) => offset += written,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(error) => return Err(BoundedFixedRecordError::Io(error.kind())),
        }
    }
    Ok(())
}

/// Reads exactly one encoded record under caller-owned bounds.
pub(crate) fn read_exact_bounded(
    stream: &mut TcpStream,
    record: &mut [u8],
    io_slice: Duration,
    total_deadline: Duration,
    cancelled: &AtomicBool,
) -> Result<(), BoundedFixedRecordError> {
    let started = Instant::now();
    let mut offset = 0;
    while offset < record.len() {
        if cancelled.load(Ordering::Acquire) {
            return Err(BoundedFixedRecordError::Cancelled);
        }
        let remaining = total_deadline
            .checked_sub(started.elapsed())
            .ok_or(BoundedFixedRecordError::Deadline)?;
        stream
            .set_read_timeout(Some(remaining.min(io_slice)))
            .map_err(|error| BoundedFixedRecordError::Io(error.kind()))?;
        match stream.read(&mut record[offset..]) {
            Ok(0) => return Err(BoundedFixedRecordError::Truncated { actual: offset }),
            Ok(read) => offset += read,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(error) => return Err(BoundedFixedRecordError::Io(error.kind())),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::sync::atomic::AtomicBool;
    use std::thread;

    #[test]
    fn lslc_003e_exact_bytes_cross_the_private_core() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let writer = thread::spawn(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            write_exact_bounded(
                &mut stream,
                &[2, 3, 5, 7, 11],
                Duration::from_millis(10),
                Duration::from_secs(1),
                &AtomicBool::new(false),
            )
            .unwrap();
        });
        let (mut stream, _) = listener.accept().unwrap();
        let mut record = [0; 5];
        read_exact_bounded(
            &mut stream,
            &mut record,
            Duration::from_millis(10),
            Duration::from_secs(1),
            &AtomicBool::new(false),
        )
        .unwrap();
        writer.join().unwrap();
        assert_eq!(record, [2, 3, 5, 7, 11]);
    }

    #[test]
    fn lslc_003e_cancellation_and_truncation_are_stage_typed() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = thread::spawn(move || {
            let mut stream = TcpStream::connect(address).unwrap();
            stream.write_all(&[1, 2]).unwrap();
        });
        let (mut stream, _) = listener.accept().unwrap();
        let mut record = [0; 3];
        assert_eq!(
            read_exact_bounded(
                &mut stream,
                &mut record,
                Duration::from_millis(10),
                Duration::from_secs(1),
                &AtomicBool::new(false),
            ),
            Err(BoundedFixedRecordError::Truncated { actual: 2 })
        );
        peer.join().unwrap();

        let cancelled = AtomicBool::new(true);
        assert_eq!(
            write_exact_bounded(
                &mut stream,
                &[1],
                Duration::from_millis(10),
                Duration::from_secs(1),
                &cancelled,
            ),
            Err(BoundedFixedRecordError::Cancelled)
        );
    }
}
