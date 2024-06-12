use testing_utils::macros as utils;

fn setup() -> usize {
    42
}

#[utils::test(setup = setup)]
fn single_server(_ctx: &mut usize) {}

fn main() {}
