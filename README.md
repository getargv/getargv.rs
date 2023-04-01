<h1><img src="logo.svg" width="200" alt="getargv"></h1>

[![Rust CI](https://github.com/getargv/getargv.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/getargv/getargv.rs/actions/workflows/rust.yml)

`libgetargv` is a library that allows you to get the arguments that were passed to another running process on macOS. It is intended to provide roughly the same functionality as reading from `/proc/<pid>/cmdline` on Linux. On macOS this is done by parsing the output of the `KERN_PROCARGS2` sysctl, which is <abbr title="always, in my observation">very often</abbr> implemented [incorrectly](https://getargv.narzt.cam/hallofshame.html), due to the overlooked possibility of leading empty arguments passed to the target process. This crate is the Rust bindings for the `libgetargv` library.

## Permissions

`libgetargv` can only see processes running as the same user by default, so be sure your process runs as the desired user (`setuid`, [`launchd.plist`](x-man-page://launchd.plist), [`sudo`](x-man-page://sudo)) or can [elevate privileges](https://developer.apple.com/library/archive/documentation/Security/Conceptual/SecureCodingGuide/Articles/AccessControl.html); n.b. elevating privileges safely is [extremely complicated](https://developer.apple.com/forums/thread/708765), and will be a target of privilege escalation attacks on macOS so be extremely careful if you go this route, better to defer to the user to elevate privileges for you as needed.

## System Requirements

macOS is required as this is a macOS specific `sysctl`, even BSD does not implement it. Your system must support `sysctl` and `KERN_PROCARGS2`, which probably means macOS [10.3](https://github.com/CamJN/xnu/blob/b52f6498893f78b034e2e00b86a3e146c3720649/bsd/sys/sysctl.h#L332) or later, though I haven't tested versions older than 10.7. You'll also need a non-ancient clang (c99 is required) or you'll have to override the compiler flags with `CC`, `EXTRA_CPPFLAGS`, and `EXTRA_CFLAGS`.

## Building `getargv`

To make `getargv`:

- Install `libgetargv` to your system (see below).
- Clone this repo and run `cargo build` or
- add the crate to your `Cargo.toml` file dependencies.

```toml
[target.'cfg(target_vendor = "apple")'.dependencies]
getargv = "~0.2.0"
```

## Installing `libgetargv`

To get access to `libgetargv`, sign up for an appropriate [sponsorship tier](https://github.com/sponsors/CamJN).

Clone the `libgetargv` repo: `git clone https://github.com/getargv/getargv.git`.

Running `make install_dylib`, installs the library to the `/usr/local/` prefix by default; you can change the install location with the `PREFIX` `make` variable: `make PREFIX=/opt install_dylib`.

I'm working on building binary artifacts to install without compilation, using `pkg` installers, however even once that's done, depeding on your system, it may still be necessary to compile from source; eg. if you have built your own xnu kernel with a custom `PID_MAX` value.

## Building `libgetargv`
I've built `libgetargv` on macOS 10.7-13, using only the <abbr title="Command Line Tools">CLT</abbr> package, not the full Xcode install. If you need to override variables, do so inside the `make` command, eg: `make EXTRA_CPPFLAGS=-DMACRO EXTRA_CFLAGS=-std=c17 dylib`. If you are trying to build on a version of macOS earlier than 10.7, let me know how it goes.
