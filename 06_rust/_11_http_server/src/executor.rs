use std::{future::Future, sync::Arc};
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct Executor {
    pub rt: Arc<Runtime>,
}

impl Executor {
    pub fn new(rt: Runtime) -> Self {
        Self { rt: Arc::new(rt) }
    }

    pub async fn run<F, T>(&self, fut: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.rt.handle().spawn(fut).await.unwrap()
    }
}
