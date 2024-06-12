pub use assert_fs as fs;
pub use bin::server_cmd;
pub use essentials;
pub use surf;
pub use test_macros as macros;
pub use testcontainers;
pub use tokio;
pub use utils::{get_random_ports, random_listeners};

use futures_util::Future;

mod bin;
mod utils;

pub async fn test<C1, C2, S, SFut, E, EFut, T, TFut>(setup: S, teardown: E, test: T)
where
    S: FnOnce() -> SFut,
    SFut: Future<Output = C1>,
    T: FnOnce(C1) -> TFut,
    TFut: Future<Output = C2>,
    E: FnOnce(C2) -> EFut,
    EFut: Future<Output = ()>,
{
    let c1 = setup().await;
    let c2 = test(c1).await;
    teardown(c2).await;
}
