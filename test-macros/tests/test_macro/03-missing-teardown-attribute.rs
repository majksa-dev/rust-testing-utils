use testing_utils::macros as utils;

async fn setup() -> usize {
    42
}

#[utils::test(setup = setup)]
async fn single_server(_ctx: &mut usize) {}

fn main() {}
