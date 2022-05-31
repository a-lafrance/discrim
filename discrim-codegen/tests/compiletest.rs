use trybuild::TestCases;

#[test]
fn ui_tests() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
