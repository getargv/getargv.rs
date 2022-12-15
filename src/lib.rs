/*-
 * Copyright: see LICENSE file
 */

#![doc(html_logo_url = "https://getargv.narzt.cam/images/logo.svg")]
#![deny(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Safe Rust wrapper for the [getargv library](https://getargv.narzt.cam/).
//!
//! This crate provides a safe wrapper for the
//! [getargv-sys](https://getargv.narzt.cam/) crate which provides FFI
//! bindings to [libgetargv](https://getargv.narzt.cam/) which provides a
//! correct parser for the `KERN_PROCARGS2` `sysctl` which is how you access
//! the arguments of another process on macOS.
//!
//! This is the preferred crate for using
//! [libgetargv](https://getargv.narzt.cam/) in your Rust project.
//!
//!# External Deps
//! You must have [libgetargv](https://getargv.narzt.cam/) installed for
//! this crate to link to, it will not build/install it for you. If
//! `libgetargv.dylib` is not located in one of `clang`'s default search
//! paths, you must set the`LIBGETARGV_LIB_DIR` env var to tell `rustc`
//! where to find it, and you will either need to set the
//! `DYLD_FALLBACK_LIBRARY_PATH` env var at runtime to tell dyld where
//! to load it from, or you will need to use `install_name_tool` on your
//! binary to fixup the library load path.
//!
//!# Reason this crate is macOS only
//! On BSDs and Linuxen you can just read `/proc/$PID/cmdline` which is both
//! faster and easier than using this lib, Solaris has `pargs`, and I don't
//! use Windows so I can't support it, but you might want to look at the
//! [wmic](https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmic) tool.
//!
//! If you are writing a cross platform program, you can depend on this crate
//! only on macOS by specifying the dependency as:
//! ```toml
//! [target.'cfg(target_vendor = "apple")'.dependencies]
//! getargv = "~PKG_VERSION"
//! ```
//! <script>const version = document.querySelector('.version').innerText.match(/[0-9\.]+/)[0];
//! document.querySelectorAll('code').forEach(c=>c.innerText=c.innerText.replace('PKG_VERSION',version));
//! </script>

pub mod argv;
pub mod argvargc;
#[doc(inline)]
pub use crate::argv::*;
#[doc(inline)]
pub use crate::argvargc::*;
