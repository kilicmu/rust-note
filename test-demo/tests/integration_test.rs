mod common;

// cargo test --test filename.rs测试指定文件内的集成测试
#[test]
fn it_adds_two() {
    assert_eq!(String::from("setup"), common::setup());
}