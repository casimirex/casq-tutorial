# Lesson 21 — Choosing a backend

**Example:** [`examples/21_backends.rs`](../examples/21_backends.rs) ·
`cargo run --example 21_backends`

## The idea

So far every circuit ran on "the simulator". Real quantum development means
choosing *where* to run: a fast exact simulator while you debug, a noisy
simulator to check robustness, or eventually a real quantum processor.

casimirQ exposes these as **backends** behind one interface. Selecting a target
is just a backend id — the circuit and request are otherwise identical. That
uniformity is the whole point: the same program runs everywhere, so you develop
on a simulator and deploy to hardware without a rewrite.

Each backend advertises its **capabilities**:

- **max qubits** — how big a circuit it takes,
- **native gates** — the gate set it runs directly (others must be *transpiled*),
- **noise** — whether it models errors,
- **connectivity** — `all-to-all` (simulators) or `linear` (device-like),
- **simulated** — false only for a real processor.

The built-in backends:

| Backend | Type | Notes |
| --- | --- | --- |
| `local-simulator` | simulator | Exact, up to 24 qubits, all-to-all |
| `noisy-simulator` | simulator | Density-matrix, models noise |
| `emulated-qpu` | hardware-emulator | Device-style native set + baseline noise |
| `remote-qpu` | hardware | A real QPU — *unavailable until configured* |

The `remote-qpu` is the seam for a real provider: it appears in the list but is
unavailable until credentials are set, so nothing ever pretends to be a device
that isn't there.

## The code

```rust
use casq_sdk::backends::BackendRunOptions;

let backends = client.backends();
for b in backends.list().await? { /* b.id, b.backend_type, b.available, b.capabilities */ }

let result = backends
    .run("emulated-qpu", &bell, BackendRunOptions { shots: Some(2000), ..Default::default() })
    .await?;
println!("{:?} {:?}", result.purity(), result.native_gate_fraction());
```

## What you'll see

The same Bell circuit on two targets:

```
--- local-simulator (exact) ---
  |00> 50.4%   |11> 49.6%          purity None, native fraction 1.0

--- emulated-qpu (device noise + restricted native set) ---
  |11> 50.1%  |00> 48.8%  |01> 0.7%  |10> 0.4%   purity 0.961, native fraction 0.5
```

On the emulated device, baseline noise degrades the state (purity `0.961`, and
the forbidden `01`/`10` states appear), and the **native-gate fraction is 0.5**:
the CNOT is native but the Hadamard is not, so it would be *transpiled* to the
device's basis before running.

## Try it yourself

1. Run a GHZ preset on `local-simulator` vs `emulated-qpu`. How does purity fall
   as the circuit gets deeper (more noisy gates)?
2. Ask for a 12-qubit circuit on `emulated-qpu` (max 7). What happens, and why is
   a qubit ceiling a real hardware constraint?
3. `remote-qpu` is unavailable here. What would it take to make it real? (Hint:
   an endpoint + token — the adapter is already wired.)

## Key takeaway

Backends make "where does this run?" a one-line choice. Develop on a simulator,
validate against noise, and target hardware — all with the same circuit. This is
the bridge from today's simulators to tomorrow's quantum processors.

Back to the [overview](../README.md).
