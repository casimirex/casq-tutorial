//! Lesson 9 — Grover's search.
//!
//! To find one marked item among N = 2^n unsorted items, a classical search
//! needs ~N/2 checks on average. Grover's algorithm needs only ~sqrt(N),
//! amplifying the marked state's amplitude with each iteration until a
//! measurement almost certainly returns it.
//!
//! casq-sdk exposes Grover through the algorithms API, which builds and runs the
//! oracle + diffusion circuit server-side.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let algos = client.algorithms();

    println!("Searching 2^n items for one marked item:\n");
    println!("  n   N=2^n   optimal iters   success probability");
    for n in 2..=6 {
        let n_items = 1usize << n;
        let marked = n_items / 3; // an arbitrary item to look for
        let result = algos.grover(n, marked, None).await?;
        println!(
            "  {n}   {n_items:>5}   {:>13}   {:.4}",
            result.optimal_iterations, result.success_probability
        );
    }

    println!("\nWith the optimal iteration count the marked item dominates,");
    println!("using ~sqrt(N) work instead of ~N — a quadratic speedup.");
    Ok(())
}
