#![cfg(feature = "time")]
#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "linux",
    target_os = "android",
    target_os = "emscripten",
))]
#[cfg(feature = "process")]
use nix::time::clock_getcpuclockid;
use nix::time::{clock_gettime, ClockId};

#[cfg(not(target_os = "redox"))]
#[test]
pub fn test_clock_getres() {
    nix::time::clock_getres(ClockId::CLOCK_REALTIME).expect("assertion failed");
}

#[test]
pub fn test_clock_gettime() {
    clock_gettime(ClockId::CLOCK_REALTIME).expect("assertion failed");
}

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "linux",
    target_os = "android",
    target_os = "emscripten",
))]
#[cfg(feature = "process")]
#[test]
pub fn test_clock_getcpuclockid() {
    let clock_id = clock_getcpuclockid(nix::unistd::Pid::this()).unwrap();
    clock_gettime(clock_id).unwrap();
}

#[cfg(not(target_os = "redox"))]
#[test]
pub fn test_clock_id_res() {
    ClockId::CLOCK_REALTIME.res().unwrap();
}

#[test]
pub fn test_clock_id_now() {
    ClockId::CLOCK_REALTIME.now().unwrap();
}

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "linux",
    target_os = "android",
    target_os = "emscripten",
))]
#[cfg(feature = "process")]
#[test]
pub fn test_clock_id_pid_cpu_clock_id() {
    ClockId::pid_cpu_clock_id(nix::unistd::Pid::this())
        .map(ClockId::now)
        .unwrap()
        .unwrap();
}
