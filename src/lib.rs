use getargv_sys as ffi;
use libc::free;
use std::{
    ffi::{c_char, c_void, CStr, OsString},
    fmt,
    io::{Error, Result},
    mem,
    os::unix::ffi::OsStringExt,
    vec,
};

pub struct Argv {
    _buffer: *const c_char,
    start_pointer: *const c_char,
    end_pointer: *const c_char,
}

impl Argv {
    fn new(buf: *mut c_char, start_pointer: *const c_char, end_pointer: *const c_char) -> Self {
        Self {
            _buffer: buf,
            start_pointer,
            end_pointer,
        }
    }

    pub fn print(&self) -> Result<()> {
        if unsafe {
            ffi::print_argv_of_pid(
                self.start_pointer as *mut c_char,
                self.end_pointer as *mut c_char,
            )
        } {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Drop for Argv {
    fn drop(&mut self) {
        unsafe { free(self._buffer as *mut c_void) }
    }
}

impl From<ffi::ArgvResult> for Argv {
    fn from(result: ffi::ArgvResult) -> Self {
        Self::new(result.buffer, result.start_pointer, result.end_pointer)
    }
}

pub fn get_argv_of_pid(pid: ffi::pid_t, nuls: bool, skip: ffi::uint) -> Result<Argv> {
    let options = ffi::GetArgvOptions { skip, pid, nuls };
    let mut result: ffi::ArgvResult = unsafe { mem::zeroed() };
    let succeeded: bool = unsafe { ffi::get_argv_of_pid(&options, &mut result) };
    if succeeded {
        Ok(Argv::from(result))
    } else {
        Err(Error::last_os_error())
    }
}

pub struct ArgvArgc {
    args: *const *const c_char,
    _count: ffi::uint,
    _buffer: *const c_char,
    iter: vec::IntoIter<OsString>,
}

impl ArgvArgc {
    fn new(buf: *const c_char, argv: *mut *const c_char, argc: ffi::uint) -> Self {
        Self {
            _buffer: buf,
            args: argv,
            _count: argc,
            iter: (0..argc as isize)
                .map(|i|
                     OsStringExt::from_vec(unsafe { CStr::from_ptr(*argv.offset(i)).to_bytes().to_vec() })
                )
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }
}

// not stable yet
//impl !Send for ArgvArgc {}
//impl !Sync for ArgvArgc {}

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
        unsafe {
            free(self.args as *mut c_void);
            free(self._buffer as *mut c_void);
        }
    }
}

impl From<ffi::ArgvArgcResult> for ArgvArgc {
    fn from(result: ffi::ArgvArgcResult) -> Self {
        Self::new(
            result.buffer,
            result.argv as *mut *const c_char,
            result.argc,
        )
    }
}

pub fn get_argv_and_argc_of_pid(pid: ffi::pid_t) -> Result<ArgvArgc> {
    let mut result: ffi::ArgvArgcResult = unsafe { mem::zeroed() };
    let succeeded: bool = unsafe { ffi::get_argv_and_argc_of_pid(pid, &mut result) };
    if succeeded {
        Ok(ArgvArgc::from(result))
    } else {
        Err(Error::last_os_error())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    #[test]
    fn get_argv_of_pid_sanity_check() {
        let result = get_argv_of_pid(process::id().try_into().unwrap(), false, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn get_argv_and_argc_of_pid_sanity_check() {
        let result = get_argv_and_argc_of_pid(process::id().try_into().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn argvargc_iter() {
        let iter = get_argv_and_argc_of_pid(process::id().try_into().unwrap()).unwrap();
        let args = std::env::args();
        args.zip(iter).for_each(|(a, e)| assert_eq!(OsString::from(a),e));
    }
}
