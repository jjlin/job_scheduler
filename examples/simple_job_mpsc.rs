use job_scheduler_ng::{Job, JobScheduler};
use std::sync::mpsc::{channel, Receiver, Sender};
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

    // Create a Send/Receive channel using a String
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // Create the receiver thread and print when we receive something
    std::thread::Builder::new()
        .name(String::from("channel-receiver"))
        .spawn(move || {
            println!(
                "{:?} - Starting channel receiver loop within thread",
                chrono::Utc::now()
            );
            loop {
                if let Ok(msg) = rx.recv() {
                    println!("{:?} - rx: {msg}", chrono::Utc::now());
                }
                std::thread::sleep(Duration::from_millis(500));
            }
        })
        .expect("Error spawning channel-receiver thread");

    // Create a job which sends a message via the channel
    sched.add(Job::new("*/5 * * * * *".parse().unwrap(), {
        move || {
            tx.send(String::from(
                "I get executed every 5 seconds and send an mpsc!",
            ))
            .unwrap();
        }
    }));

    std::thread::Builder::new()
        .name(String::from("job-scheduler"))
        .spawn(move || {
            println!(
                "{:?} - Starting job scheduler loop within thread",
                chrono::Utc::now()
            );
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
}
