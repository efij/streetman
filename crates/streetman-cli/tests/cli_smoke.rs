use std::process::{Command, Stdio};

#[test]
fn cli_compress_smoke() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "compress",
            "--mode",
            "full",
            "--domain",
            "prose",
            "--no-archive",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn streetman");
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"The database configuration should be checked before deployment.")
            .expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("dtbs") || stdout.contains("cnfgrtn"));
}
