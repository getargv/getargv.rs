use clap::{command, value_parser, Arg, ArgAction, ArgMatches};
use getargv::get_argv_of_pid;
use libc::pid_t;
use std::{
    ffi::c_uint as uint,
    io::{stdout, Write},
};

const fn arg_max() -> usize {
    if option_env!("DEP_GETARGV_MACOSX_DEPLOYMENT_TARGET").is_some() {
        1024 * 1024
    } else {
        1024 * 256
    }
}

#[cfg(feature = "const_int_from_str")]
const fn pid_max() -> pid_t {
    match pid_t::from_str_radix(env!("DEP_GETARGV_PID_MAX"), 10) {
        Ok(p) => p,
        _ => panic!("DEP_GETARGV_PID_MAX env var was missing or unparseable"),
    }
}

#[cfg(not(feature = "const_int_from_str"))]
fn pid_max() -> pid_t {
    env!("DEP_GETARGV_PID_MAX").parse::<pid_t>().unwrap()
}

fn parse_args() -> (pid_t, usize, bool) {
    let matches: ArgMatches = command!()
        .arg(
            Arg::new("pid")
                .required(true)
                .help("The pid of the process for which to get the arguments")
                .value_name("PID")
                .value_parser(value_parser!(u64).range(0..=pid_max() as u64))
                .index(1),
        )
        .arg(
            Arg::new("skip")
                .short('s')
                .value_name("skip")
                .value_parser(value_parser!(u64).range(0..=arg_max() as u64))
                .default_value("0")
                .required(false)
                .help("Number of arguments to skip"),
        )
        .arg(
            Arg::new("nuls")
                .short('0')
                .help("Output args NUL separated")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let pid = *matches.get_one::<u64>("pid").expect("PID is required") as pid_t;
    let skip: usize = *matches.get_one::<u64>("skip").unwrap() as usize;
    let nuls = matches.get_flag("nuls");
    (pid, skip, nuls)
}

fn main() {
    let (pid, skip, nuls) = parse_args();

    let argv = get_argv_of_pid(pid, nuls, skip as uint);
    argv.expect("failed to get args")
        .print()
        .expect("failed to print");
    let _ = stdout().flush();
}
