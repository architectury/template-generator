use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}

#[cfg(target_arch = "wasm32")]
pub fn block_on<F: Future>(future: F) -> F::Output {
    futures::executor::block_on(future)
}
