#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/parse_from.rs");
    t.compile_fail("tests/build/enum_from.rs");
    t.compile_fail("tests/build/unsupported_from.rs");
}
