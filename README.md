# Malory

Malory is a rust async benchmarking tool using Tokio.

```
use malory;

#[tokio::main]
fn main() {
	let iterations = 1000;
	let parallelism = 5;
	let context = "That's how you get ants.";

	malory::judge_me(iterations, parallelism, context, |ctx, i| async { true }).await;
}
```
