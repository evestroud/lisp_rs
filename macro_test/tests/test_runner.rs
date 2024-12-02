#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/no_functions.rs");
    t.compile_fail("tests/incorrect_signature.rs");
    t.pass("tests/compiles.rs");
}
