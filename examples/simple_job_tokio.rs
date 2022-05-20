use job_scheduler_ng::{Job, JobScheduler};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Initializing scheduler!");
    init_scheduler().await;

    let wait_seconds: u64 = 40;
    println!("Waiting for {wait_seconds} seconds!");
    tokio::time::sleep(Duration::from_secs(wait_seconds)).await;
    println!("Finished. Goodby!");
}

async fn init_scheduler() {
    // Start a new runtime to not mess with the current running one
    let runtime = tokio::runtime::Runtime::new().unwrap();

    std::thread::Builder::new()
        .name("job-scheduler".to_string())
        .spawn(move || {
            let _runtime_guard = runtime.enter();

            let mut sched = JobScheduler::new();

            sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
                println!(
                    "{:?} - I get executed every 10 seconds!",
                    chrono::Utc::now()
                );
            }));

            sched.add(Job::new("*/4 * * * * *".parse().unwrap(), || {
                println!("{:?} - I get executed every 4 seconds!", chrono::Utc::now());
            }));

            sched.add(Job::new("*/5 * * * * *".parse().unwrap(), || {
                runtime.spawn(test_job_every_five());
            }));

            sched.add(Job::new("*/8 * * * * *".parse().unwrap(), || {
                runtime.spawn(test_job_every_eight());
            }));

            println!("{:?} - Starting loop", chrono::Utc::now());
            loop {
                sched.tick();
                runtime.block_on(async move {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                });
            }
        })
        .expect("Error spawing job-scheduler thread");
}

async fn test_job_every_five() {
    println!(
        "{:?} - I get executed every 5 seconds (begin)!",
        chrono::Utc::now()
    );
    // Wait 6 seconds, this will demonstrate this call will be async and not blocking.
    tokio::time::sleep(Duration::from_secs(6)).await;
    println!(
        "{:?} - I get executed every 5 seconds (end)!",
        chrono::Utc::now()
    );
}

async fn test_job_every_eight() {
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("{:?} - I get executed every 8 seconds!", chrono::Utc::now());
}
