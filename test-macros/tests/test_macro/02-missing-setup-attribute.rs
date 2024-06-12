use testing_utils::macros as utils;

fn teardown(ctx: usize) {
    println!("Tearing down: {}", ctx);
}

#[utils::test(teardown = teardown)]
fn single_server(_ctx: &mut usize) {}

fn main() {}
