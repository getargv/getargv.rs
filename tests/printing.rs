use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::process::Command;

#[test]
fn print_works() {
    let bin = env!("CARGO_BIN_EXE_test_child");
    let args = ["zero", "one", "two", "three"];
    let mut cmd = Command::new(bin);
    if let Some((first, rest)) = args.split_first() {
        cmd.arg0(first).args(rest);
        for (nuls, sep) in [(true, " "), (false, "\0")] {
            for s in 0..=args.len() {
                let output = cmd
                    .env("TEST_CHILD_SKIP", s.to_string())
                    .env("TEST_CHILD_NULS", nuls.to_string())
                    .output()
                    .expect("failed to execute process");
                let expected = OsString::from(
                    args.split_at(s).1.join(sep) + if s < args.len() { "\0" } else { "" },
                );
                assert_eq!(OsStr::from_bytes(&output.stdout), expected.as_os_str());
            }
        }
    }
}
