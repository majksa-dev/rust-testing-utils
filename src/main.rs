use essentials::info;

fn main() {
    essentials::install();
    let result = testing_utils::add(2, 2);
    info!("Result: {}", result);
}
