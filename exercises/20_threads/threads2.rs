// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().expect("Failed to obtain lock");
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let status = status.lock().expect("Failed to obtain lock");
    println!("Jobs completed: {}", status.jobs_completed);
}
