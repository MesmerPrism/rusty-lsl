// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private, bounded admission for caller-requested Float32 report post-processing.
//!
//! Admission only binds already completed report ownership, the caller's explicit
//! request, and one exact caller-owned sequence per retained record. It performs no
//! processing or activation and infers no loss or compatibility property.

use crate::requested_timestamp_post_processing::RequestedTimestampPostProcessing;
use crate::TimestampedFloat32InletSessionReport;

/// Configuration refusal before an admission owner exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallerRequestedFloat32ReportPostProcessingAdmissionConfigError {
    ZeroMaximumRecords,
    MaximumRecordsUnrepresentable { requested: usize },
}

/// Typed admission refusal retaining every caller-owned input.
#[derive(Debug)]
pub(crate) enum CallerRequestedFloat32ReportPostProcessingAdmissionError {
    EmptyReport {
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    },
    SequenceExtentMismatch {
        request: RequestedTimestampPostProcessing,
        sequence_count: usize,
        report_record_count: usize,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    },
    RecordLimitExceeded {
        request: RequestedTimestampPostProcessing,
        maximum_records: usize,
        report_record_count: usize,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    },
    AllocationRefused {
        request: RequestedTimestampPostProcessing,
        requested: usize,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    },
}

/// Owned, validation-complete input for a future processing entrypoint.
#[derive(Debug)]
pub(crate) struct CallerRequestedFloat32ReportPostProcessingPlan {
    maximum_records: usize,
    request: RequestedTimestampPostProcessing,
    sequences: Vec<u64>,
    report: TimestampedFloat32InletSessionReport,
}

impl CallerRequestedFloat32ReportPostProcessingPlan {
    pub(crate) const fn maximum_records(&self) -> usize {
        self.maximum_records
    }

    pub(crate) const fn request(&self) -> RequestedTimestampPostProcessing {
        self.request
    }

    pub(crate) fn record_count(&self) -> usize {
        self.report.record_count()
    }

    pub(crate) fn sequences(&self) -> &[u64] {
        &self.sequences
    }

    pub(crate) const fn report(&self) -> &TimestampedFloat32InletSessionReport {
        &self.report
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        usize,
        RequestedTimestampPostProcessing,
        Vec<u64>,
        TimestampedFloat32InletSessionReport,
    ) {
        (
            self.maximum_records,
            self.request,
            self.sequences,
            self.report,
        )
    }
}

/// Sole private owner of the P35 pre-processing admission boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CallerRequestedFloat32ReportPostProcessingAdmission {
    maximum_records: usize,
}

impl CallerRequestedFloat32ReportPostProcessingAdmission {
    pub(crate) fn new(
        maximum_records: usize,
    ) -> Result<Self, CallerRequestedFloat32ReportPostProcessingAdmissionConfigError> {
        if maximum_records == 0 {
            return Err(
                CallerRequestedFloat32ReportPostProcessingAdmissionConfigError::ZeroMaximumRecords,
            );
        }
        if u64::try_from(maximum_records).is_err() {
            return Err(CallerRequestedFloat32ReportPostProcessingAdmissionConfigError::
                MaximumRecordsUnrepresentable {
                    requested: maximum_records,
                });
        }
        Ok(Self { maximum_records })
    }

    pub(crate) const fn maximum_records(self) -> usize {
        self.maximum_records
    }

    pub(crate) fn admit(
        self,
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
        report: TimestampedFloat32InletSessionReport,
    ) -> Result<
        CallerRequestedFloat32ReportPostProcessingPlan,
        CallerRequestedFloat32ReportPostProcessingAdmissionError,
    > {
        let report_record_count = report.record_count();
        match admit_owned(
            self.maximum_records,
            request,
            sequences,
            report,
            report_record_count,
            |target, requested| target.try_reserve_exact(requested).map_err(|_| ()),
        ) {
            Ok((request, sequences, report)) => {
                Ok(CallerRequestedFloat32ReportPostProcessingPlan {
                    maximum_records: self.maximum_records,
                    request,
                    sequences,
                    report,
                })
            }
            Err(refusal) => Err(refusal.into_public()),
        }
    }
}

#[derive(Debug)]
enum AdmissionRefusal<T> {
    Empty {
        request: RequestedTimestampPostProcessing,
        sequences: Vec<u64>,
        report: T,
    },
    Mismatch {
        request: RequestedTimestampPostProcessing,
        sequence_count: usize,
        extent: usize,
        sequences: Vec<u64>,
        report: T,
    },
    Limit {
        request: RequestedTimestampPostProcessing,
        maximum: usize,
        extent: usize,
        sequences: Vec<u64>,
        report: T,
    },
    Allocation {
        request: RequestedTimestampPostProcessing,
        requested: usize,
        sequences: Vec<u64>,
        report: T,
    },
}

impl AdmissionRefusal<TimestampedFloat32InletSessionReport> {
    fn into_public(self) -> CallerRequestedFloat32ReportPostProcessingAdmissionError {
        match self {
            Self::Empty {
                request,
                sequences,
                report,
            } => CallerRequestedFloat32ReportPostProcessingAdmissionError::EmptyReport {
                request,
                sequences,
                report,
            },
            Self::Mismatch {
                request,
                sequence_count,
                extent,
                sequences,
                report,
            } => CallerRequestedFloat32ReportPostProcessingAdmissionError::SequenceExtentMismatch {
                request,
                sequence_count,
                report_record_count: extent,
                sequences,
                report,
            },
            Self::Limit {
                request,
                maximum,
                extent,
                sequences,
                report,
            } => CallerRequestedFloat32ReportPostProcessingAdmissionError::RecordLimitExceeded {
                request,
                maximum_records: maximum,
                report_record_count: extent,
                sequences,
                report,
            },
            Self::Allocation {
                request,
                requested,
                sequences,
                report,
            } => CallerRequestedFloat32ReportPostProcessingAdmissionError::AllocationRefused {
                request,
                requested,
                sequences,
                report,
            },
        }
    }
}

fn admit_owned<T, F>(
    maximum: usize,
    request: RequestedTimestampPostProcessing,
    mut sequences: Vec<u64>,
    report: T,
    extent: usize,
    reserve: F,
) -> Result<(RequestedTimestampPostProcessing, Vec<u64>, T), AdmissionRefusal<T>>
where
    F: FnOnce(&mut Vec<u64>, usize) -> Result<(), ()>,
{
    if extent == 0 {
        return Err(AdmissionRefusal::Empty {
            request,
            sequences,
            report,
        });
    }
    if sequences.len() != extent {
        return Err(AdmissionRefusal::Mismatch {
            request,
            sequence_count: sequences.len(),
            extent,
            sequences,
            report,
        });
    }
    if extent > maximum {
        return Err(AdmissionRefusal::Limit {
            request,
            maximum,
            extent,
            sequences,
            report,
        });
    }
    let mut admitted = Vec::new();
    if reserve(&mut admitted, extent).is_err() {
        return Err(AdmissionRefusal::Allocation {
            request,
            requested: extent,
            sequences,
            report,
        });
    }
    admitted.append(&mut sequences);
    Ok((request, admitted, report))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct ReportToken(u64);

    fn pass() -> RequestedTimestampPostProcessing {
        RequestedTimestampPostProcessing::PassThrough
    }

    #[test]
    fn zero_and_platform_u64_boundary_are_exact() {
        assert_eq!(
            CallerRequestedFloat32ReportPostProcessingAdmission::new(0),
            Err(CallerRequestedFloat32ReportPostProcessingAdmissionConfigError::ZeroMaximumRecords)
        );
        let largest = usize::try_from(u64::MAX).unwrap_or(usize::MAX);
        assert_eq!(
            CallerRequestedFloat32ReportPostProcessingAdmission::new(largest)
                .unwrap()
                .maximum_records(),
            largest
        );
        if usize::BITS > u64::BITS {
            let too_large = usize::try_from(u64::MAX).unwrap().checked_add(1).unwrap();
            assert_eq!(CallerRequestedFloat32ReportPostProcessingAdmission::new(too_large), Err(CallerRequestedFloat32ReportPostProcessingAdmissionConfigError::MaximumRecordsUnrepresentable { requested: too_large }));
        }
    }

    #[test]
    fn empty_mismatch_and_upper_bound_preserve_exact_sequences() {
        for (maximum, extent, sequences, kind) in [
            (3, 0, vec![], 0),
            (3, 2, vec![u64::MIN], 1),
            (1, 2, vec![u64::MIN, u64::MAX], 2),
        ] {
            let original = sequences.clone();
            let error = admit_owned(
                maximum,
                pass(),
                sequences,
                ReportToken(9),
                extent,
                |target, count| target.try_reserve_exact(count).map_err(|_| ()),
            )
            .unwrap_err();
            let returned = match error {
                AdmissionRefusal::Empty { sequences, .. } if kind == 0 => sequences,
                AdmissionRefusal::Mismatch {
                    sequence_count: 1,
                    extent: 2,
                    sequences,
                    ..
                } if kind == 1 => sequences,
                AdmissionRefusal::Limit {
                    maximum: 1,
                    extent: 2,
                    sequences,
                    ..
                } if kind == 2 => sequences,
                _ => panic!("unexpected refusal"),
            };
            assert_eq!(returned, original);
        }
    }

    #[test]
    fn local_allocation_refusal_has_no_global_state_and_preserves_ownership() {
        let sequences = vec![4, u64::MAX];
        let pointer = sequences.as_ptr();
        let error = admit_owned(2, pass(), sequences, ReportToken(17), 2, |_, requested| {
            assert_eq!(requested, 2);
            Err(())
        })
        .unwrap_err();
        match error {
            AdmissionRefusal::Allocation {
                requested,
                sequences,
                report,
                ..
            } => {
                assert_eq!(requested, 2);
                assert_eq!(sequences, vec![4, u64::MAX]);
                assert_eq!(sequences.as_ptr(), pointer);
                assert_eq!(report, ReportToken(17));
            }
            _ => panic!("unexpected refusal"),
        }
    }

    #[test]
    fn success_owns_one_extreme_sequence_per_exact_record_without_reordering() {
        let sequences = vec![u64::MIN, 1, u64::MAX];
        let (request, admitted, report) = admit_owned(
            3,
            pass(),
            sequences,
            ReportToken(u64::MAX),
            3,
            |target, count| target.try_reserve_exact(count).map_err(|_| ()),
        )
        .unwrap();
        assert_eq!(request, pass());
        assert_eq!(admitted, vec![u64::MIN, 1, u64::MAX]);
        assert_eq!(admitted.len(), 3);
        assert_eq!(report, ReportToken(u64::MAX));
    }

    #[test]
    fn checks_do_not_attempt_impossible_usize_allocations() {
        let error = admit_owned(
            usize::MAX,
            pass(),
            vec![7],
            ReportToken(1),
            usize::MAX,
            |_, _| panic!("extent mismatch must precede allocation"),
        )
        .unwrap_err();
        assert!(
            matches!(error, AdmissionRefusal::Mismatch { sequence_count: 1, extent: usize::MAX, sequences, .. } if sequences == vec![7])
        );
    }
}
