//! Shared helpers for the casq-tutorial examples.
//!
//! Every lesson connects to a running casimirQ server the same way, so that
//! boilerplate lives here. Configure the target with environment variables
//! (all optional, with sensible local-Docker defaults):
//!
//! - `CASQ_BASE_URL`  (default `http://localhost:8080/api/v1`)
//! - `CASQ_EMAIL`     (default `admin@example.com`)
//! - `CASQ_PASSWORD`  (default `admin123`)

use casq_sdk::{Client, Result};
use std::collections::HashMap;

/// Connect to the casimirQ API and log in, returning an authenticated client.
pub async fn connect() -> Result<Client> {
    let base_url =
        std::env::var("CASQ_BASE_URL").unwrap_or_else(|_| "http://localhost:8080/api/v1".into());
    let email = std::env::var("CASQ_EMAIL").unwrap_or_else(|_| "admin@example.com".into());
    let password = std::env::var("CASQ_PASSWORD").unwrap_or_else(|_| "admin123".into());

    let mut client = Client::new(base_url)?;
    client.login(&email, &password).await?;
    Ok(client)
}

/// Read the value of qubit `q` out of a big-endian measurement bitstring
/// (the rightmost character is qubit 0).
pub fn qubit_bit(bitstring: &str, q: usize) -> Option<char> {
    bitstring.chars().rev().nth(q)
}

/// Print measurement counts as a sorted ASCII bar chart.
pub fn print_histogram(counts: &HashMap<String, u64>) {
    let total: u64 = counts.values().sum();
    let max = counts.values().copied().max().unwrap_or(1);

    let mut rows: Vec<(&String, &u64)> = counts.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    for (state, count) in rows {
        let bar_len = (*count as f64 / max as f64 * 40.0).round() as usize;
        let pct = if total > 0 {
            *count as f64 / total as f64 * 100.0
        } else {
            0.0
        };
        println!(
            "  |{state}>  {:>6}  {:>5.1}%  {}",
            count,
            pct,
            "#".repeat(bar_len)
        );
    }
}
