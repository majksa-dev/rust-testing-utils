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

pub async fn test<C, S, SFut, E, EFut, T, TFut>(setup: S, teardown: E, test: T)
where
    C: Sized + Send + 'static,
    S: FnOnce() -> SFut,
    SFut: Future<Output = C>,
    T: FnOnce(&mut C) -> TFut,
    TFut: Future<Output = ()>,
    E: FnOnce(C) -> EFut,
    EFut: Future<Output = ()>,
{
    let mut ctx = setup().await;
    test(&mut ctx).await;
    teardown(ctx).await;
}
