use std::process::Command;

#[test]
fn valid_file() {
    let output = Command::new("target/debug/json")
        .arg("tests/valid.json")
        .output()
        .expect("failed");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout),
"bar = [
  1 -> true
  2 -> null
  3 -> [\"a\",null,false,{\"hoge\":1}]
  4 = [
    a -> \"A\"
    b -> \"B\"
  ]
]
baz = [
]
foo -> 0
"   );
}

#[test]
fn valid_file2() {
    let output = Command::new("target/debug/json")
        .arg("tests/valid2.json")
        .output()
        .expect("failed");

    assert!(output.status.success());
}

#[test]
fn invalid_file() {
    let output = Command::new("target/debug/json")
        .arg("tests/invalid.json")
        .output()
        .expect("failed");

    assert!(!output.status.success());
}

#[test]
fn arg() {
    let output = Command::new("target/debug/json")
        .output()
        .expect("failed");

    assert!(!output.status.success());
}
