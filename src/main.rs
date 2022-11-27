use getargv::get_argv_of_pid;
use getargv_sys::pid_t;
use std::process;
use std::env::var;
use std::env::VarError;

fn main(){
    let nuls = var("TEST_CHILD_NULS").map(|s|s.eq_ignore_ascii_case("true")).unwrap_or(false);
    let skip = var("TEST_CHILD_SKIP").and_then(|s|s.parse::<u32>().map_err(|_e|VarError::NotPresent)).unwrap_or(0);
    let pid: pid_t = process::id().try_into().unwrap();
    if let Ok(argv) = get_argv_of_pid(pid, nuls, skip) {
        let _res = argv.print();
    }
}
