use std::env;

#[test]
fn test_macro() {
    env::set_var("RUST_BACKTRACE", "0");
    let t = trybuild::TestCases::new();
    t.pass("tests/test_macro/01-parse-attributes.rs");
    t.compile_fail("tests/test_macro/02-missing-setup-attribute.rs");
    t.compile_fail("tests/test_macro/03-missing-teardown-attribute.rs");
}

#[cfg_attr(not(feature = "disabled-feature"), ignore)]
#[test_macros::test(setup = setup, teardown = teardown)]
async fn single_server(_: usize) -> usize {
    panic!("This test should not run");
}

async fn setup() -> usize {
    42
}

async fn teardown(ctx: usize) {
    println!("Tearing down: {}", ctx);
}
