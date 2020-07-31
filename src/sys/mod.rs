use libc;

#[cfg(unix)]
pub const AF_INET = lib.AF_INET

#[cfg(windows)]
pub const AF_INET : libc::c_int = 2;

#[cfg(windows)]
pub const AF_INET6: libc::c_int = 10;