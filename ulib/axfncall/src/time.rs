use arceos_api::modules::axhal::time::monotonic_time;
use arceos_api::modules::axlog::debug;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct TimeSpec {
    tv_sec: usize,
    tv_nsec: usize,
}
pub(crate) unsafe fn abi_timespec(ts: *mut TimeSpec) {
    let ts = &mut *ts;
    let now = monotonic_time();
    ts.tv_nsec = now.as_nanos() as usize;
    ts.tv_sec = now.as_secs() as usize;
    debug!("{:?}", ts);
}