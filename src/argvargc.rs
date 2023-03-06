/*-
 * Copyright: see LICENSE file
 */

//! Provides an [Iterator] over the arguments of a process.
//!
//! Provides a function [get_argv_and_argc_of_pid] that parses the args of a process
//! and then returns a [struct][ArgvArgc] that allows you to iterate over them.

use getargv_sys as ffi;
use std::{
    ffi::{CStr, OsString},
    fmt,
    mem,
    os::unix::ffi::OsStringExt,
    io::{Error, Result},
    vec,
};

/// Contains an iterable representation of the arguments as parsed by [get_argv_and_argc_of_pid].
pub struct ArgvArgc {
    res: ffi::ArgvArgcResult,
    iter: vec::IntoIter<OsString>,
}

impl ArgvArgc {
    fn new(res: ffi::ArgvArgcResult) -> Self {
        Self {
            iter: (0..res.argc as isize)
                .map(|i| {
                    OsStringExt::from_vec(unsafe {
                        CStr::from_ptr(*res.argv.offset(i)).to_bytes().to_vec()
                    })
                })
                .collect::<Vec<_>>()
                .into_iter(),
            res,
        }
    }
}

/* Something can safely be Send unless it shares mutable state with something else
 * without enforcing exclusive access to it. Each ArgvArgc has a unique buffer and pointers
 * into it, so we're good.
 */
unsafe impl Send for ArgvArgc {}

/* For ArgvArgc to be Sync we have to enforce that you can't write to something stored in a
 * &ArgvArgc while that same something could be read or written to from another &ArgvArgc. Since
 * ArgvArgc can be modified by the iterator methods, this needs careful consideration. I've seen
 * that structs that impl Iterator seem sync so it's probably fine.
 */
unsafe impl Sync for ArgvArgc {}

impl Default for ArgvArgc {
    fn default() -> Self {
        Self::new(ffi::ArgvArgcResult::default())
    }
}

impl fmt::Debug for ArgvArgc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.as_slice().fmt(f)
    }
}

impl Iterator for ArgvArgc {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl ExactSizeIterator for ArgvArgc {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl DoubleEndedIterator for ArgvArgc {
    fn next_back(&mut self) -> Option<OsString> {
        self.iter.next_back()
    }
}

impl Drop for ArgvArgc {
    fn drop(&mut self) {
        unsafe { ffi::free_ArgvArgcResult(&mut self.res); }
    }
}

/// Parses the arguments of a process, and on success returns an [Iterator] over them.
///
///# Argument
/// * `pid` - the process id of the other process to target
///
///# Examples
///```
/// # use getargv::get_argv_and_argc_of_pid;
///if let Ok(argvargc) = get_argv_and_argc_of_pid(unsafe{libc::getppid()}) {
///  println!("We got our parent process' arguments as an iterator! There are {} of them.", argvargc.len());
///}
///```
pub fn get_argv_and_argc_of_pid(pid: ffi::pid_t) -> Result<ArgvArgc> {
    let mut result: ffi::ArgvArgcResult = unsafe { mem::zeroed() };
    let succeeded: bool = unsafe { ffi::get_argv_and_argc_of_pid(pid, &mut result) };
    if succeeded {
        Ok(ArgvArgc::new(result))
    } else {
        Err(Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;
    use std::fmt::Write;
    use std::ptr::null_mut;

    #[test]
    fn get_argv_and_argc_of_pid_sanity_check() {
        let result = get_argv_and_argc_of_pid(process::id().try_into().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn argvargc_default_trait_sanity_check() {
        let argv_argc: ArgvArgc = Default::default();
        assert_eq!(argv_argc.res.argc, 0);
        assert_eq!(argv_argc.res.buffer, null_mut());
        assert_eq!(argv_argc.res.argv, null_mut());
        assert_eq!(argv_argc.len(), 0);
        assert_eq!(argv_argc.last(), None);
    }

    #[test]
    fn argvargc_debug_trait_sanity_check() {
        let argv_argc: ArgvArgc = Default::default();
        let mut output = String::new();
        write!(&mut output, "{:?}",argv_argc)
            .expect("Error occurred while trying to write in String");
        assert_eq!(output, "[]");
    }

    #[test]
    fn argvargc_iterator_trait_sanity_check() {
        let mut argv_argc: ArgvArgc = Default::default();
        assert_eq!(argv_argc.next(), None);
    }

    #[test]
    fn argvargc_iterator_trait_works() {
        let iter = get_argv_and_argc_of_pid(process::id().try_into().unwrap()).unwrap();
        let args = std::env::args_os();
        assert!(args.eq(iter));
    }

    #[test]
    fn argvargc_exact_size_iterator_trait_sanity_check() {
        let argv_argc: ArgvArgc = Default::default();
        assert_eq!(argv_argc.len(), 0);
    }

    #[test]
    fn argvargc_double_ended_iterator_trait_sanity_check() {
        let mut argv_argc: ArgvArgc = Default::default();
        assert_eq!(argv_argc.nth_back(0), None);
    }

}
