//! Raw ICMP socket

#![deny(missing_docs)]

mod compat;
mod socket;

#[cfg(unix)]
#[path = "sys/unix.rs"]
mod sys;

#[cfg(windows)]
#[path = "sys/windows.rs"]
mod sys;

pub use socket::IcmpSocket;

#[cfg(test)]
mod tests;
