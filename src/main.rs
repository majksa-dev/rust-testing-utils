use essentials::info;

fn main() {
    essentials::install();
    let cmd = testing_utils::server_cmd();
    info!("Command: {:?}", cmd);
}
