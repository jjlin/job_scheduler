use job_scheduler_ng::{Job, JobScheduler};
use std::time::Duration;

fn main() {
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

    std::thread::Builder::new()
        .name(String::from("job-scheduler"))
        .spawn(move || {
            println!("{:?} - Starting loop within thread", chrono::Utc::now());
            loop {
                sched.tick();
                std::thread::sleep(Duration::from_millis(500));
            }
        })
        .expect("Error spawning job-scheduler thread");

    let wait_seconds: u64 = 40;
    println!("Waiting for {wait_seconds} seconds!");
    std::thread::sleep(Duration::from_secs(wait_seconds));
    println!("Finished. Goodby!");
    std::process::exit(0);
}
