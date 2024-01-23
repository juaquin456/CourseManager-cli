use std::process::Command;

#[test]
fn test_create_and_list() {
    let res = Command::new("./target/debug/course_manager")
        .args(["-w", "/tmp", "create", "cycle", "1", "2"])
        .output()
        .expect("error");
    assert!(res.status.success());

    let res = Command::new("cargo")
        .args(["run", "list", "cycles"])
        .output()
        .expect("error");
    assert!(res.status.success());
    assert!(String::from_utf8_lossy(&res.stdout).contains("1-2"));

    let res = Command::new("cargo")
        .args(["run", "remove", "cycle", "1", "2"])
        .output()
        .expect("error");
    assert!(res.status.success());
}
