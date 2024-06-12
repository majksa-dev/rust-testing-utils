use testing_utils::macros as utils;

async fn setup() -> usize {
    42
}

async fn teardown(ctx: usize) {
    println!("Tearing down: {}", ctx);
}

#[utils::test(setup = setup, teardown = teardown)]
async fn single_server(_ctx: &mut usize) {}

fn main() {}
