use testing_utils::macros as utils;

async fn setup() -> usize {
    42
}

#[utils::test(setup = setup)]
async fn single_server(ctx: usize) {
    ctx
}

fn main() {}
