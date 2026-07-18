//! Lesson 22 — Asynchronous execution.
//!
//! Real quantum runs don't finish instantly: a device has a queue, and a big
//! simulation takes time. Instead of blocking, you *submit* a job, get an id
//! back immediately, and poll it until it finishes. This is exactly how you'd
//! talk to real hardware.
//!
//! We submit the same Bell circuit two ways — on the default runner and, async,
//! on the emulated QPU — and wait for each to complete.

use casq_sdk::jobs::{SubmitJobOptions, WaitOptions};
use casq_sdk::Circuit;
use casq_tutorial::{connect, print_histogram};

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let jobs = client.jobs();

    let mut bell = Circuit::new(2);
    bell.h(0).cx(0, 1);

    // 1. Submit — returns immediately with a queued job.
    let queued = jobs
        .submit(&bell, SubmitJobOptions { shots: Some(2000), ..Default::default() })
        .await?;
    println!("submitted {} (status: {:?})", queued.id, queued.status);

    // 2. Wait for it to finish (polls in the background).
    let done = jobs.wait_for(&queued.id, WaitOptions::default()).await?;
    println!("finished with status {:?}", done.status);
    if let Some(result) = &done.result {
        print_histogram(result.counts());
        println!("statevector entries: {}", result.results.statevector.len());
    }

    // 3. Submit the same circuit to the emulated QPU, asynchronously.
    println!("\nSame job, targeting the emulated QPU:");
    let on_qpu = jobs
        .submit(
            &bell,
            SubmitJobOptions {
                backend_id: Some("emulated-qpu".into()),
                shots: Some(2000),
                ..Default::default()
            },
        )
        .await?;
    let done = jobs.wait_for(&on_qpu.id, WaitOptions::default()).await?;
    if let Some(result) = &done.result {
        print_histogram(result.counts());
        println!(
            "ran on backend {:?}, statevector entries: {} (backends return counts, not a statevector)",
            result.backend_id(),
            result.results.statevector.len()
        );
    }

    // Tidy up.
    jobs.delete(&queued.id).await?;
    jobs.delete(&on_qpu.id).await?;
    Ok(())
}
