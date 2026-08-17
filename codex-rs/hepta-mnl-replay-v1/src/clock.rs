use rustix::time::ClockId;
use rustix::time::Timespec;

use crate::ReplayStoreResultV1;
use crate::error::invalid;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ClockSampleV1 {
    pub(crate) boottime_after: Timespec,
    pub(crate) boottime_before: Timespec,
    pub(crate) realtime: Timespec,
}

impl ClockSampleV1 {
    pub(crate) fn from_system() -> ReplayStoreResultV1<Self> {
        let sample = Self {
            boottime_before: rustix::time::clock_gettime(ClockId::Boottime),
            realtime: rustix::time::clock_gettime(ClockId::Realtime),
            boottime_after: rustix::time::clock_gettime(ClockId::Boottime),
        };
        sample.validate()?;
        Ok(sample)
    }

    pub(crate) fn validate(self) -> ReplayStoreResultV1<()> {
        validate_timespec(self.boottime_before, "CLOCK_BOOTTIME before REALTIME")?;
        validate_timespec(self.realtime, "CLOCK_REALTIME")?;
        validate_timespec(self.boottime_after, "CLOCK_BOOTTIME after REALTIME")?;
        if self.boottime_before > self.boottime_after {
            return Err(invalid(
                "CLOCK_BOOTTIME moved backwards inside one supervisor sample",
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_sample_sequence(
    before: ClockSampleV1,
    after: ClockSampleV1,
) -> ReplayStoreResultV1<()> {
    before.validate()?;
    after.validate()?;
    if before.boottime_after > after.boottime_before {
        return Err(invalid(
            "CLOCK_BOOTTIME moved backwards across durable replay publication",
        ));
    }
    if before.realtime > after.realtime {
        return Err(invalid(
            "CLOCK_REALTIME moved backwards across durable replay publication",
        ));
    }
    Ok(())
}

pub(crate) fn validate_realtime_window(
    sample: ClockSampleV1,
    not_before_unix_seconds: u64,
    expires_at_unix_seconds: u64,
) -> ReplayStoreResultV1<()> {
    sample.validate()?;
    let not_before = i64::try_from(not_before_unix_seconds)
        .map_err(|_| invalid("signed not-before is not representable by CLOCK_REALTIME"))?;
    let expires_at = i64::try_from(expires_at_unix_seconds)
        .map_err(|_| invalid("signed expiry is not representable by CLOCK_REALTIME"))?;
    if sample.realtime.tv_sec < not_before || sample.realtime.tv_sec >= expires_at {
        return Err(invalid(
            "CLOCK_REALTIME is outside the signed half-open freshness window",
        ));
    }
    Ok(())
}

fn validate_timespec(value: Timespec, label: &str) -> ReplayStoreResultV1<()> {
    if value.tv_sec < 0 || !(0..1_000_000_000).contains(&value.tv_nsec) {
        return Err(invalid(format!(
            "{label} is not a canonical nonnegative timespec"
        )));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) const fn sample_for_tests(
    boottime_before_seconds: i64,
    boottime_before_nanoseconds: i64,
    realtime_seconds: i64,
    realtime_nanoseconds: i64,
    boottime_after_seconds: i64,
    boottime_after_nanoseconds: i64,
) -> ClockSampleV1 {
    ClockSampleV1 {
        boottime_before: Timespec {
            tv_sec: boottime_before_seconds,
            tv_nsec: boottime_before_nanoseconds,
        },
        realtime: Timespec {
            tv_sec: realtime_seconds,
            tv_nsec: realtime_nanoseconds,
        },
        boottime_after: Timespec {
            tv_sec: boottime_after_seconds,
            tv_nsec: boottime_after_nanoseconds,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_open_realtime_boundaries_are_exact() {
        let at_start = sample_for_tests(1, 0, 100, 0, 1, 1);
        validate_realtime_window(at_start, 100, 101).expect("not-before is inclusive");
        let just_before_expiry = sample_for_tests(2, 0, 100, 999_999_999, 2, 1);
        validate_realtime_window(just_before_expiry, 100, 101)
            .expect("last nanosecond before expiry");
        let at_expiry = sample_for_tests(3, 0, 101, 0, 3, 1);
        assert!(validate_realtime_window(at_expiry, 100, 101).is_err());
    }

    #[test]
    fn malformed_and_backwards_samples_are_rejected_without_nanosecond_math() {
        assert!(
            sample_for_tests(1, 0, 100, 1_000_000_000, 1, 1)
                .validate()
                .is_err()
        );
        assert!(sample_for_tests(-1, 0, 100, 0, 1, 1).validate().is_err());
        assert!(sample_for_tests(2, 0, 100, 0, 1, 0).validate().is_err());

        let before = sample_for_tests(10, 0, 100, 1, 10, 1);
        let realtime_rollback = sample_for_tests(11, 0, 100, 0, 11, 1);
        assert!(validate_sample_sequence(before, realtime_rollback).is_err());
        let boottime_rollback = sample_for_tests(9, 0, 101, 0, 9, 1);
        assert!(validate_sample_sequence(before, boottime_rollback).is_err());
    }
}
