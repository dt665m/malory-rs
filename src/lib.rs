use std::future::Future;
use std::sync::Arc;

use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant};

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
        // tokio 1 differences
        // let tx = tx.clone();
        let mut tx = tx.clone();
        tokio::spawn(async move {
            // tokio 1 differences
            // let _permit = sem.acquire().await.expect("semaphore is open. qed");
            let _permit = sem.acquire().await;
            let start = Instant::now();
            let success = run(ctx, i).await;
            let elapsed = std::cmp::max(Duration::from_micros(1), Instant::now() - start);
            println!("task {:?} completed in {:?}", i, elapsed);
            tx.send(success).await.unwrap();
        });
    }
    drop(tx);

    while let Some(success) = rx.recv().await {
        // println!("handling results");
        if success {
            successful += 1;
        }
    }

    println!("successful results {:?}", successful);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let test = "test";
        judge_me(10, 2, test, |a, i| async { true }).await;
    }
}
