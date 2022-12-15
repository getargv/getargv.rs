use getargv::get_argv_of_pid;
use clap::{Arg, command, ArgAction, ArgMatches, value_parser};
use libc::pid_t;
use std::{
    io::{stdout, Write},
    ffi::c_uint as uint
};

const ARG_MAX: usize = 1024 * 1024;
const PID_MAX: pid_t = 99_999;

fn parse_args() -> (pid_t, usize, bool) {
    let matches: ArgMatches = command!()
        .arg(Arg::new("pid")
             .required(true)
             .help("The pid of the process for which to get the arguments")
             .value_name("PID")
             .value_parser(value_parser!(u64).range(0..=PID_MAX as u64))
             .index(1)
        )
        .arg(Arg::new("skip")
             .short('s')
             .value_name("skip")
             .value_parser(value_parser!(u64).range(0..ARG_MAX as u64))
             .default_value("0")
             .required(false)
             .help("Number of arguments to skip"),
        )
        .arg(Arg::new("nuls")
             .short('0')
             .help("Output args NUL separated")
             .required(false)
             .action(ArgAction::SetTrue))
        .get_matches();

    let pid = *matches.get_one::<u64>("pid").expect("PID is required") as pid_t;
    let skip: usize = *matches.get_one::<u64>("skip").unwrap() as usize;
    let nuls = matches.get_flag("nuls");
    (pid, skip, nuls)
}

fn main() {
    let (pid, skip, nuls) = parse_args();

    let argv = get_argv_of_pid(pid, nuls, skip as uint);
    argv.expect("failed to get args").print().expect("failed to print");
    let _ = stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_get_arg_max() {
        assert!(true);
    }
}
