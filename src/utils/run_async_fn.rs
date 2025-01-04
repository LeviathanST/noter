use std::{future::Future, thread};

use tokio::runtime::{self, Handle};

pub fn run_async_fn_on_current_thread<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(future)
}

pub fn run_async_fn<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    let handle = Handle::current();

    thread::spawn(move || handle.block_on(future))
        .join()
        .unwrap()
}
