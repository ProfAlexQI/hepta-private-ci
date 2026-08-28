use std::io;
use std::num::NonZeroU64;

const BYTES_PER_MIB: u64 = 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MemoryBudget {
    limit_bytes: NonZeroU64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("resident memory {observed_bytes} bytes exceeded limit {limit_bytes} bytes")]
pub(crate) struct MemoryBudgetExceeded {
    pub(crate) limit_bytes: u64,
    pub(crate) observed_bytes: u64,
}

impl MemoryBudget {
    pub(crate) fn from_mib(memory_limit_mib: u64) -> Result<Self, io::Error> {
        let limit_bytes = memory_limit_mib
            .checked_mul(BYTES_PER_MIB)
            .and_then(NonZeroU64::new)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "memory_limit_mib must be non-zero and fit in bytes",
                )
            })?;
        Ok(Self { limit_bytes })
    }

    pub(crate) fn limit_bytes(self) -> u64 {
        self.limit_bytes.get()
    }

    pub(crate) fn check(self, observed_bytes: u64) -> Result<(), MemoryBudgetExceeded> {
        if observed_bytes > self.limit_bytes() {
            return Err(MemoryBudgetExceeded {
                limit_bytes: self.limit_bytes(),
                observed_bytes,
            });
        }
        Ok(())
    }

    pub(crate) fn install_hard_limit(self) -> Result<InstalledMemoryLimit, io::Error> {
        platform::install_hard_limit(self.limit_bytes())
    }
}

pub(crate) fn resident_memory_bytes() -> Result<u64, io::Error> {
    platform::resident_memory_bytes()
}

pub(crate) struct InstalledMemoryLimit {
    #[cfg(windows)]
    job: windows_sys::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
impl Drop for InstalledMemoryLimit {
    fn drop(&mut self) {
        if !self.job.is_null() {
            unsafe {
                windows_sys::Win32::Foundation::CloseHandle(self.job);
            }
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
mod platform {
    use std::io;

    use super::InstalledMemoryLimit;

    pub(super) fn install_hard_limit(limit_bytes: u64) -> Result<InstalledMemoryLimit, io::Error> {
        let requested = libc::rlim_t::try_from(limit_bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "memory byte limit does not fit rlim_t",
            )
        })?;
        let mut current = std::mem::MaybeUninit::<libc::rlimit>::uninit();
        if unsafe { libc::getrlimit(libc::RLIMIT_AS, current.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let mut current = unsafe { current.assume_init() };
        let effective = if current.rlim_max == libc::RLIM_INFINITY {
            requested
        } else {
            requested.min(current.rlim_max)
        };
        if current.rlim_cur == libc::RLIM_INFINITY || current.rlim_cur > effective {
            current.rlim_cur = effective;
            if unsafe { libc::setrlimit(libc::RLIMIT_AS, &current) } != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(InstalledMemoryLimit {})
    }

    #[cfg(target_os = "linux")]
    pub(super) fn resident_memory_bytes() -> Result<u64, io::Error> {
        let status = std::fs::read_to_string("/proc/self/status")?;
        let line = status
            .lines()
            .find(|line| line.starts_with("VmRSS:"))
            .ok_or_else(|| io::Error::other("/proc/self/status has no VmRSS field"))?;
        let kib = line
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| io::Error::other("VmRSS value is missing"))?
            .parse::<u64>()
            .map_err(|error| io::Error::other(format!("invalid VmRSS value: {error}")))?;
        kib.checked_mul(1024)
            .ok_or_else(|| io::Error::other("VmRSS byte count overflowed"))
    }

    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    pub(super) fn resident_memory_bytes() -> Result<u64, io::Error> {
        let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let usage = unsafe { usage.assume_init() };
        let resident = u64::try_from(usage.ru_maxrss)
            .map_err(|_| io::Error::other("resident memory value was negative"))?;
        #[cfg(target_os = "macos")]
        {
            Ok(resident)
        }
        #[cfg(target_os = "freebsd")]
        {
            resident
                .checked_mul(1024)
                .ok_or_else(|| io::Error::other("resident memory byte count overflowed"))
        }
    }
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;
    use std::io;

    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;
    use windows_sys::Win32::System::JobObjects::CreateJobObjectW;
    use windows_sys::Win32::System::JobObjects::JOBOBJECT_EXTENDED_LIMIT_INFORMATION;
    use windows_sys::Win32::System::JobObjects::JOB_OBJECT_LIMIT_PROCESS_MEMORY;
    use windows_sys::Win32::System::JobObjects::JobObjectExtendedLimitInformation;
    use windows_sys::Win32::System::JobObjects::SetInformationJobObject;
    use windows_sys::Win32::System::ProcessStatus::K32GetProcessMemoryInfo;
    use windows_sys::Win32::System::ProcessStatus::PROCESS_MEMORY_COUNTERS;
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    use super::InstalledMemoryLimit;

    pub(super) fn install_hard_limit(limit_bytes: u64) -> Result<InstalledMemoryLimit, io::Error> {
        let process_limit = usize::try_from(limit_bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "memory byte limit does not fit usize",
            )
        })?;
        let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if job.is_null() {
            return Err(io::Error::last_os_error());
        }
        let mut information = unsafe {
            std::mem::zeroed::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()
        };
        information.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_MEMORY;
        information.ProcessMemoryLimit = process_limit;
        let set = unsafe {
            SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                (&raw const information).cast::<c_void>(),
                u32::try_from(std::mem::size_of_val(&information)).map_err(|_| {
                    io::Error::other("job object information size does not fit u32")
                })?,
            )
        };
        if set == 0 {
            let error = io::Error::last_os_error();
            unsafe {
                CloseHandle(job);
            }
            return Err(error);
        }
        if unsafe { AssignProcessToJobObject(job, GetCurrentProcess()) } == 0 {
            let error = io::Error::last_os_error();
            unsafe {
                CloseHandle(job);
            }
            return Err(error);
        }
        Ok(InstalledMemoryLimit { job })
    }

    pub(super) fn resident_memory_bytes() -> Result<u64, io::Error> {
        let mut counters = unsafe { std::mem::zeroed::<PROCESS_MEMORY_COUNTERS>() };
        counters.cb = u32::try_from(std::mem::size_of_val(&counters))
            .map_err(|_| io::Error::other("process memory counter size does not fit u32"))?;
        if unsafe {
            K32GetProcessMemoryInfo(GetCurrentProcess(), &raw mut counters, counters.cb)
        } == 0
        {
            return Err(io::Error::last_os_error());
        }
        u64::try_from(counters.WorkingSetSize)
            .map_err(|_| io::Error::other("working set size does not fit u64"))
    }
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    windows
)))]
mod platform {
    use std::io;

    use super::InstalledMemoryLimit;

    pub(super) fn install_hard_limit(_limit_bytes: u64) -> Result<InstalledMemoryLimit, io::Error> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "hard memory limits are unsupported on this platform",
        ))
    }

    pub(super) fn resident_memory_bytes() -> Result<u64, io::Error> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "resident memory observation is unsupported on this platform",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mib_conversion_is_exact_and_non_zero() {
        let budget = MemoryBudget::from_mib(64).expect("valid memory budget");
        assert_eq!(budget.limit_bytes(), 64 * BYTES_PER_MIB);
        assert!(MemoryBudget::from_mib(0).is_err());
    }

    #[test]
    fn resident_memory_check_fails_only_above_limit() {
        let budget = MemoryBudget::from_mib(1).expect("valid memory budget");
        assert!(budget.check(BYTES_PER_MIB).is_ok());
        assert_eq!(
            budget.check(BYTES_PER_MIB + 1),
            Err(MemoryBudgetExceeded {
                limit_bytes: BYTES_PER_MIB,
                observed_bytes: BYTES_PER_MIB + 1,
            })
        );
    }
}
