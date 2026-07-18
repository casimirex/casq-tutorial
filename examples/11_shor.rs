//! Lesson 11 — Shor's factoring algorithm.
//!
//! Shor's algorithm factors an integer N in polynomial time by reducing
//! factoring to *period finding*, which a quantum computer solves efficiently
//! using the QFT. This is the result that threatens RSA: a large enough
//! fault-tolerant machine could break today's public-key cryptography.
//!
//! We factor a few small numbers through the algorithms API and check the
//! factors multiply back to N.

use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let algos = client.algorithms();

    for n in [15u64, 21, 35] {
        let r = algos.shor(n).await?;
        let product: i64 = r.factors.iter().product();
        let nontrivial = r.factors.iter().all(|&f| f != 1 && f as u64 != n);
        println!(
            "N = {n:>2}  ->  factors {:?}  (product {product}, attempts {}, nontrivial: {nontrivial})",
            r.factors, r.attempts
        );
    }

    println!("\nFactoring is easy to *check* but classically hard to *do* at scale —");
    println!("that asymmetry is exactly what Shor's algorithm attacks.");
    Ok(())
}
