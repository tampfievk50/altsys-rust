pub struct ProcessUtil;

impl ProcessUtil {
    /// Checks whether a previously-spawned detached process is still alive,
    /// so the runner can skip firing a job whose last run hasn't finished yet.
    #[cfg(unix)]
    pub fn is_running(pid: i32) -> bool {
        unsafe { libc::kill(pid, 0) == 0 }
    }

    #[cfg(not(unix))]
    pub fn is_running(_pid: i32) -> bool {
        false
    }
}
