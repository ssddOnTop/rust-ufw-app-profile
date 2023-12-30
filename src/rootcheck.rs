//! [![crates.io](https://img.shields.io/crates/v/sudo?logo=rust)](https://crates.io/crates/sudo/)
//! [![docs.rs](https://docs.rs/sudo/badge.svg)](https://docs.rs/sudo)
//!
//! Detect if you are running as root, restart self with `sudo` if needed or setup uid zero when running with the SUID flag set.
//!
//! ## Requirements
//!
//! * The `sudo` program is required to be installed and setup correctly on the target system.
//! * Linux or Mac OS X tested
//!     * It should work on *BSD. However, it is not tested.
#![allow(clippy::bool_comparison)]

use libc::setuid;
use log::trace;
use std::process::Command;

/// Cross platform representation of the state the current program running
#[derive(Debug, PartialEq)]
pub enum RunningAs {
    /// Root (Linux/Mac OS/Unix) or Administrator (Windows)
    Root,
    /// Running as a normal user
    User,
    /// Started from SUID, a call to `sudo::escalate_if_needed` or `sudo::with_env` is required to claim the root privileges at runtime.
    /// This does not restart the process.
    Suid,
}
use RunningAs::*;

#[cfg(unix)]
/// Check getuid() and geteuid() to learn about the configuration this program is running under
pub fn check() -> RunningAs {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    match (uid, euid) {
        (0, 0) => Root,
        (_, 0) => Suid,
        (_, _) => User,
    }
    //if uid == 0 { Root } else { User }
}

#[cfg(unix)]
#[inline]
pub fn escalate_if_needed() -> bool {
    with_env(&[])
}

#[cfg(unix)]
pub fn with_env(prefixes: &[&str]) -> bool {
    let current = check();
    trace!("Running as {:?}", current);
    match current {
        Root => {
            trace!("already running as Root");
            return true;
            // return Ok(current);
        }
        Suid => {
            trace!("setuid(0)");
            unsafe {
                setuid(0);
            }
            return true;
            // return Ok(current);
        }
        User => {
            log::debug!("Escalating privileges");
        }
    }

    let mut args: Vec<_> = std::env::args().collect();
    if let Some(absolute_path) = std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|p| p.to_string()))
    {
        args[0] = absolute_path;
    }
    let mut command: Command = Command::new("/usr/bin/sudo");

    // Always propagate RUST_BACKTRACE
    if let Ok(trace) = std::env::var("RUST_BACKTRACE") {
        let value = match &*trace.to_lowercase() {
            "" => None,
            "1" | "true" => Some("1"),
            "full" => Some("full"),
            invalid => {
                log::warn!(
                    "RUST_BACKTRACE has invalid value {:?} -> defaulting to \"full\"",
                    invalid
                );
                Some("full")
            }
        };
        if let Some(value) = value {
            trace!("relaying RUST_BACKTRACE={}", value);
            command.arg(format!("RUST_BACKTRACE={}", value));
        }
    }

    if prefixes.is_empty() == false {
        for (name, value) in std::env::vars().filter(|(name, _)| name != "RUST_BACKTRACE") {
            if prefixes.iter().any(|prefix| name.starts_with(prefix)) {
                trace!("propagating {}={}", name, value);
                command.arg(format!("{}={}", name, value));
            }
        }
    }

    let mut child = command.args(args).spawn().expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");

    ecode.success()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = check();
    }
}
