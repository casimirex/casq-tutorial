# Lesson 23 — Asynchronous execution

**Example:** [`examples/23_async_jobs.rs`](../examples/23_async_jobs.rs) ·
`cargo run --example 23_async_jobs`

## The idea

Everything so far ran *synchronously*: call `run`, wait, get the answer. That's
fine for a small local simulation — but real quantum execution doesn't work that
way. A device has a **queue**; a large simulation takes **minutes**. Blocking a
request thread for that is a non-starter.

The **async job engine** is the answer. You **submit** a circuit and get a job id
back immediately (the job starts in `queued`). The server runs it in the
background — optionally on a chosen **backend** (Lesson 22). You then **poll** the
job until it settles into a terminal state: `completed`, `failed`, or `cancelled`.

This is the same shape you'd use against real hardware: submit, walk away, come
back for the result.

## The code

```rust
use casq_sdk::jobs::{SubmitJobOptions, WaitOptions};

// Submit — returns at once with a queued job.
let job = client.jobs().submit(&bell, SubmitJobOptions {
    backend_id: Some("emulated-qpu".into()),   // async run on any backend
    shots: Some(2000),
    ..Default::default()
}).await?;

// Block until it finishes (polls under the hood).
let done = client.jobs().wait_for(&job.id, WaitOptions::default()).await?;
if let Some(result) = done.result {
    println!("{:?}", result.counts());
}
```

`wait_for` polls at an interval until the job is terminal or a timeout elapses;
for finer control, poll `jobs().get(id)` yourself and read `status` / `progress`.
Jobs can be `cancel`led (while queued) and `delete`d.

## What you'll see

The same Bell circuit, run async two ways:

```
submitted job-... (status: Queued)
finished with status Completed
  |00> 50.3%   |11> 49.7%          statevector entries: 2

Same job, targeting the emulated QPU:
  |11> 50.1%  |00> 48.8%  |01> 0.7%  |10> 0.4%
  ran on backend Some("emulated-qpu"), statevector entries: 0
```

On the default runner the job returns an exact result (with a statevector); on
the emulated QPU it returns noisy counts and no statevector — but **both went
through the same submit/wait flow**. Where a circuit runs is a one-line choice;
*how* you run it (async) is identical.

## Try it yourself

1. Submit several jobs in a loop, then `list` them and print each `status` and
   `progress`.
2. Submit a job and immediately `cancel` it. What status does it end in?
3. Lower `WaitOptions.timeout` to something tiny. What error do you get, and why
   is a timeout the right thing for a queued job that never runs?

## Key takeaway

Async submission is how quantum programs talk to anything slower than a local
simulator — a big job, a noisy backend, or a real device queue. Combined with
backends (Lesson 22), you can now dispatch a circuit to any target and collect
the result whenever it's ready. That's the shape of production quantum software.

Congratulations — you've gone from a single qubit to submitting asynchronous
jobs across multiple backends. Back to the [overview](../README.md).
