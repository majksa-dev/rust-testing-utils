use testing_utils::macros as utils;

async fn teardown(ctx: usize) {
    println!("Tearing down: {}", ctx);
}

#[utils::test(teardown = teardown)]
async fn single_server(_ctx: &mut usize) {}

fn main() {}
