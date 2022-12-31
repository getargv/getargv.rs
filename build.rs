/*-
 * Copyright: see LICENSE file
 */

use std::env;

fn cross_compiling() -> bool {
    //                                    host is reflected in cfg!             target is reflected in env
    #[cfg(feature = "is_some_and")]{      cfg!(target_vendor = "apple") ^ env::var("CARGO_CFG_TARGET_VENDOR").is_ok_and(|v| v == "apple")}
    #[cfg(not(feature = "is_some_and"))]{ cfg!(target_vendor = "apple") ^ (env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple")}
    //                                        note the operator is xor (^) not a ligature of && (∧)
}

fn ensure_apple() {
    if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() != "apple" {
        panic!("The KERN_PROCARGS2 sysctl only exists in xnu kernels, BSD or Linux users should just read /proc/$PID/cmdline which is much easier and faster, Solaris users should use pargs.\nIf you are writing a cross platform program, you can depend on this crate only on macOS by specifying the dependency as:\n[target.'cfg(target_vendor = \"apple\")'.dependencies]\n{} = \"{}\"",env!("CARGO_PKG_NAME"),env!("CARGO_PKG_VERSION"))
    }
}

fn debug_env() {
    env::vars().for_each(|(key, value)| println!("cargo:warning={}={}", key, value));
}

fn reexport_env(key: &str){
    println!("cargo:rustc-env={}={}",key,env::var(key).unwrap());
}

fn main() {
    ensure_apple();
    if env::var_os("DEBUG_CARGO_ENV").is_some() {
        debug_env();
    }
    if cross_compiling() {

    }
    let building_docs = env::var("DOCS_RS").unwrap_or_else(|_| "0".to_string()) == "1";
    if !building_docs {
        reexport_env("DEP_GETARGV_MACOSX_DEPLOYMENT_TARGET");
        reexport_env("DEP_GETARGV_PID_MAX");
    }
}
