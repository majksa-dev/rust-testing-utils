use std::{path, process::Command};

fn cargo_bin() -> Option<path::PathBuf> {
    let name = cargo_name()?;
    let env_var = format!("CARGO_BIN_EXE_{}", name);
    std::env::var_os(env_var).map(|p| p.into())
}

fn cargo_name() -> Option<String> {
    std::env::var_os("CARGO_PKG_NAME").map(|s| s.to_string_lossy().into_owned())
}

pub fn server_cmd() -> Command {
    match cargo_bin() {
        Some(cmd) => Command::new(cmd),
        None => {
            let mut cmd = Command::new("cargo");
            cmd.arg("run").arg("--release");
            cmd
        }
    }
}
