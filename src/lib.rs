pub mod app;

pub mod aulog;

static LOG_PATH: &str = "/var/log/audit";

// check uid is 0
pub fn is_root() -> bool {
    0 == unsafe { libc::getuid() }
}
