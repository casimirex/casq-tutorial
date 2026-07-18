//! Lesson 3 — Single-qubit gates.
//!
//! Every single-qubit gate is a rotation of the Bloch sphere. This lesson runs
//! a handful of the fundamental gates and reports the resulting state so you can
//! build intuition for what each one does.
//!
//! - X: bit flip           |0> -> |1>
//! - Z: phase flip          |+> -> |->  (invisible to a Z-basis measurement)
//! - H: basis change        |0> -> |+>
//! - S: quarter phase turn
//! - Rx/Ry/Rz: continuous rotations by an angle

use casq_sdk::{Circuit, Engine, RunOptions};
use casq_tutorial::connect;

#[tokio::main]
async fn main() -> casq_sdk::Result<()> {
    let client = connect().await?;
    let opts = || RunOptions::new().engine(Engine::Statevector).shots(1);

    // Helper closure that builds a one-qubit circuit, runs it, and prints state.
    async fn show(
        client: &casq_sdk::Client,
        label: &str,
        build: impl FnOnce(&mut Circuit),
    ) -> casq_sdk::Result<()> {
        let mut c = Circuit::new(1);
        build(&mut c);
        let r = client
            .run(&c, RunOptions::new().engine(Engine::Statevector).shots(1))
            .await?;
        let amps: Vec<String> = r
            .statevector()
            .iter()
            .map(|a| format!("{:+.3}{:+.3}i|{}>", a.re, a.im, a.state))
            .collect();
        println!("{label:<28} {}", amps.join("  "));
        Ok(())
    }

    let _ = opts; // opts() kept for reference in the lesson text

    println!("Starting from |0>, each gate produces:");
    show(&client, "X (bit flip)", |c| { c.x(0); }).await?;
    show(&client, "H (superposition)", |c| { c.h(0); }).await?;
    show(&client, "H then Z then H", |c| { c.h(0).z(0).h(0); }).await?;
    show(&client, "S after H", |c| { c.h(0).s(0); }).await?;
    show(&client, "Rx(pi/2)", |c| { c.rx(0, std::f64::consts::FRAC_PI_2); }).await?;
    show(&client, "Ry(pi/2)", |c| { c.ry(0, std::f64::consts::FRAC_PI_2); }).await?;

    println!("\nTip: X = H·Z·H — a bit flip is a phase flip viewed in the X basis.");
    Ok(())
}
