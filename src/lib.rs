use std::future::Future;
use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant};

/// Spawn multiple ```run``` futures and benchmark elapsed time
pub async fn judge_me<C, F, Fut>(count: usize, parallelism: usize, context: C, run: F)
where
    C: Clone + Send + Sync + 'static,
    F: Fn(C, usize) -> Fut + Send + Sync + Copy + 'static,
    Fut: Future<Output = bool> + Send,
{
    let mut successful: usize = 0;
    let semaphore = Arc::new(Semaphore::new(parallelism));
    let (tx, mut rx): (Sender<bool>, Receiver<bool>) = channel(count);

    for i in 0..count {
        let sem = semaphore.clone();
        let ctx = context.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let _permit = sem.acquire().await.expect("semaphore is open. qed");
            let start = Instant::now();
            let success = run(ctx, i).await;
            let elapsed = std::cmp::max(Duration::from_micros(1), Instant::now() - start);
            log::debug!("task {:?} completed in {:?}", i, elapsed);
            tx.send(success).await.unwrap();
        });
    }
    drop(tx);

    while let Some(success) = rx.recv().await {
        log::debug!("handling results");
        if success {
            successful += 1;
        }
    }

    log::debug!("successful results {:?}", successful);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let test = "test";
        judge_me(10, 2, test, |_a, _i| async { true }).await;
    }
}
