use testing_utils::macros as utils;

fn setup() -> usize {
    42
}

fn teardown(ctx: usize) {
    println!("Tearing down: {}", ctx);
}

#[utils::test(setup = setup, teardown = teardown)]
fn single_server(_ctx: &mut usize) {}

fn main() {}
