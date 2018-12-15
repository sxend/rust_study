use futures::future::Future;
use futures_cpupool::Builder;
use std::thread;

fn main() {
    let pool = Builder::new()
        .pool_size(2)
        .name_prefix("prefixed_thread_pool")
        .create();
    let f0 = pool.spawn_fn(|| task("task 0"));
    let f1 = pool.spawn_fn(|| task("task 1"));
    let f2 = pool.spawn_fn(|| task("task 2"));

    let result = f0.join(f1).join(f2).wait();
    println!(
        "futures result: [{:?}]. current thread: {}",
        result,
        current_thread_name()
    );
}

fn task(name: &str) -> Result<String, String> {
    Ok(format!(
        "task: [{}]. finish in thread: {}",
        name,
        current_thread_name()
    ))
}

fn current_thread_name() -> String {
    thread::current().name().unwrap_or("unknown").to_string()
}
