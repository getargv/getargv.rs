/*-
 * Copyright: see LICENSE file
 */

//! Fast way to obtain and [print][Argv::print] a representation of the arguments of a process.
//!
//! Provides a function [get_argv_of_pid] that parses the args of a process
//! and then returns a [struct][Argv] that allows you to [print][Argv::print] them.

use getargv_sys as ffi;
use std::{
    ffi::c_char,
    io::{Error, Result},
    mem,
};

/// Contains a printable representation of the arguments as parsed by [get_argv_of_pid].
#[derive(Debug)]
pub struct Argv(ffi::ArgvResult);

impl Argv {
    /// Prints the arguments as parsed by [get_argv_of_pid].
    ///
    /// # Example
    /// ```rust
    /// # use getargv::get_argv_of_pid;
    ///if let Ok(argv) = get_argv_of_pid(unsafe{libc::getppid()}, false, 0) {
    ///  assert!(argv.print().is_ok());
    ///}
    /// ```
    pub fn print(&self) -> Result<()> {
        if unsafe {
            ffi::print_argv_of_pid(
                self.0.start_pointer as *const c_char,
                self.0.end_pointer as *const c_char,
            )
        } {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Returns the length of the args to be printed in bytes
    ///
    /// # Example
    /// ```rust
    /// # use getargv::get_argv_of_pid;
    ///if let Ok(argv) = get_argv_of_pid(unsafe{libc::getppid()}, false, 0) {
    ///  println!("{}",argv.len());
    ///}
    /// ```
    pub fn len(&self) -> usize {
        if self.0.start_pointer.is_null() || self.0.end_pointer.is_null() {
            0
        } else {
            unsafe { self.0.end_pointer.offset_from(self.0.start_pointer).try_into().unwrap() }
        }
    }

    /// Returns whether the Argv is empty
    ///
    /// # Example
    /// ```rust
    /// # use getargv::get_argv_of_pid;
    ///if let Ok(argv) = get_argv_of_pid(unsafe{libc::getppid()}, false, 0) {
    ///  println!("{}",argv.is_empty());
    ///}
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for Argv {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

/* Something can safely be Send unless it shares mutable state with something else
 * without enforcing exclusive access to it. Each Argv has a unique buffer and pointers
 * into it, so we're good.
 */
unsafe impl Send for Argv {}

/* For Argv to be Sync we have to enforce that you can't write to something stored in an
 * &Argv while that same something could be read or written to from another &Argv. Since
 * Argv doesn't have public members, nor any methods that modify it, there are no
 * soundness issues making Argv sync either.
 */
unsafe impl Sync for Argv {}

impl Drop for Argv {
    fn drop(&mut self) {
        unsafe { ffi::free_ArgvResult(&mut self.0) }
    }
}

/// Parses the arguments of another process into a printable format.
///
///# Arguments
///
/// * `pid` - the process id of the other process to target
/// * `nuls` - when printing, replace ␀ separators with spaces (when `true`)
/// * `skip` - when printing, skip this number of leading arguments
///
///# Examples
///```rust
/// # use getargv::get_argv_of_pid;
///if let Ok(argv) = get_argv_of_pid(unsafe{libc::getppid()}, false, 0) {
///  println!("We got our parent process' arguments, null separated, and without skipping any!");
///}
///```
///```rust
/// # use getargv::get_argv_of_pid;
///if let Ok(argv) = get_argv_of_pid(unsafe{libc::getppid()}, true, 1) {
///  println!("We got our parent process' arguments, space separated, and skipping the first one!");
///}
///```
pub fn get_argv_of_pid(pid: ffi::pid_t, nuls: bool, skip: ffi::uint) -> Result<Argv> {
    let options = ffi::GetArgvOptions { skip, pid, nuls };
    let mut result: ffi::ArgvResult = Default::default();
    let succeeded: bool = unsafe { ffi::get_argv_of_pid(&options, &mut result) };
    if succeeded {
        Ok(Argv(result))
    } else {
        Err(Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::ESRCH;
    use std::process;
    use std::fmt::Write;

    #[test]
    fn get_argv_of_pid_sanity_check_ok() {
        let result = get_argv_of_pid(process::id().try_into().unwrap(), false, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn get_argv_of_pid_sanity_check_err() {
        let result = get_argv_of_pid(-1, false, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().raw_os_error().unwrap(), ESRCH);
    }

    #[test]
    fn argv_print_sanity_check() {
        let result = get_argv_of_pid(process::id().try_into().unwrap(), false, 0);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.print().is_ok());
    }

    #[test]
    fn argv_default_trait_sanity_check() {
        let argv: Argv = Default::default();
        assert_eq!(argv.0.buffer, std::ptr::null_mut());
        assert_eq!(argv.0.start_pointer, std::ptr::null_mut());
        assert_eq!(argv.0.end_pointer, std::ptr::null_mut());
    }

    #[test]
    fn argv_debug_trait_sanity_check() {
        let argv: Argv = Default::default();
        let mut output = String::new();
        write!(&mut output, "{:?}",argv)
            .expect("Error occurred while trying to write in String");
        assert_eq!(output, "Argv(ArgvResult { buffer: 0x0, start_pointer: 0x0, end_pointer: 0x0 })");
    }
}
