use testing_utils::server_cmd;

#[test]
fn check_bin_name() {
    let mut bin = server_cmd().get_program().to_str().unwrap().to_string();
    let last_separator = bin.rfind('/').unwrap();
    bin = bin.split_at(last_separator + 1).1.to_string();
    assert_eq!(bin, "testing-utils");
}
