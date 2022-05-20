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

    println!("{:?} - Starting loop", chrono::Utc::now());
    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
