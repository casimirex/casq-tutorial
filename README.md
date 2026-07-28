# casq-tutorial — Quantum Computing for Developers, with Rust

A hands-on, **novice-to-professional** quantum computing course built entirely
on the [`casq-sdk`](../casq-sdk) Rust client and the casimirQ simulation
platform. Every concept comes with a small, runnable Rust program you can read,
execute, and modify.

> This tutorial is original teaching material. It uses standard, well-known
> quantum algorithms (Bell states, Deutsch–Jozsa, Grover, QFT, Shor,
> teleportation, VQE, QAOA) implemented from scratch against `casq-sdk`.

## Who this is for

Developers who are comfortable with programming but new to quantum computing.
You do **not** need quantum physics or heavy math — we build intuition from
running code and add just enough theory to explain what you see.

## What you'll be able to do by the end

- Explain qubits, superposition, entanglement, and measurement in concrete terms.
- Build quantum circuits with a fluent gate API and run them on real engines.
- Implement and run the landmark quantum algorithms.
- Structure a real application on top of a quantum backend (persist circuits,
  sample results, call algorithms).

## Prerequisites

1. **Rust** (stable, 1.75+): <https://rustup.rs>
2. **A running casimirQ server.** The easiest way is the bundled Docker stack:
   ```bash
   cd ../casimirQ && docker compose up --build   # serves http://localhost:8080
   ```
3. This tutorial talks to it through `casq-sdk` (a path dependency in
   `Cargo.toml`).

## Configuration

Every lesson reads these environment variables (all optional, with local-Docker
defaults):

| Variable | Default |
| --- | --- |
| `CASQ_BASE_URL` | `http://localhost:8080/api/v1` |
| `CASQ_EMAIL` | `admin@example.com` |
| `CASQ_PASSWORD` | `admin123` |

## How to run a lesson

Each lesson is a Cargo example. Run any of them by name:

```bash
cargo run --example 01_hello_quantum
cargo run --example 10_grover
# ...
```

The written explanation for each lesson lives in [`lessons/`](./lessons); the
code lives in [`examples/`](./examples).

## The learning path

### Part I — Foundations (novice)

| # | Lesson | Example | You'll learn |
| --- | --- | --- | --- |
| 1 | [Hello, Quantum](lessons/01-hello-quantum.md) | `01_hello_quantum` | Qubits, superposition, your first circuit |
| 2 | [Measurement & probability](lessons/02-measurement.md) | `02_measurement` | Shots, counts, sampling vs. exact probabilities |
| 3 | [Single-qubit gates](lessons/03-single-qubit-gates.md) | `03_single_qubit_gates` | X/Y/Z/H/S/T and rotations as Bloch-sphere moves |

### Part II — Entanglement & tooling (intermediate)

| # | Lesson | Example | You'll learn |
| --- | --- | --- | --- |
| 4 | [Entanglement: Bell states](lessons/04-bell-state.md) | `04_bell_state` | Two-qubit entanglement and correlations |
| 5 | [GHZ & multi-qubit entanglement](lessons/05-ghz-state.md) | `05_ghz_state` | Scaling entanglement to n qubits |
| 6 | [Multi-qubit gates](lessons/06-multi-qubit-gates.md) | `06_multi_qubit_gates` | CNOT, CZ, SWAP, Toffoli, Fredkin — the coupling gates that build entanglement and logic |
| 7 | [Engines & scaling](lessons/07-engines.md) | `07_engines` | Statevector vs. Clifford, when to use which |

### Part III — Quantum algorithms (advancing)

| # | Lesson | Example | You'll learn |
| --- | --- | --- | --- |
| 8 | [Quantum RNG](lessons/08-qrng.md) | `08_qrng` | Turning superposition into real randomness |
| 9 | [Deutsch–Jozsa](lessons/09-deutsch-jozsa.md) | `09_deutsch_jozsa` | Oracles and the first quantum advantage |
| 10 | [Grover's search](lessons/10-grover.md) | `10_grover` | Quadratic speedup for unstructured search |
| 11 | [Quantum Fourier Transform](lessons/11-qft.md) | `11_qft` | The transform behind phase estimation |
| 12 | [Shor's algorithm](lessons/12-shor.md) | `12_shor` | Factoring and its impact on cryptography |
| 13 | [Quantum teleportation](lessons/13-teleportation.md) | `13_teleportation` | Moving states with entanglement + classical bits |

### Part IV — Applied & variational (professional)

| # | Lesson | Example | You'll learn |
| --- | --- | --- | --- |
| 14 | [VQE](lessons/14-vqe.md) | `14_vqe` | Hybrid algorithms for quantum chemistry |
| 15 | [QAOA](lessons/15-qaoa.md) | `15_qaoa` | Near-term quantum optimization (MaxCut) |
| 16 | [Capstone: your first quantum app](lessons/16-capstone.md) | `16_capstone` | Circuits + persistence + algorithms, end to end |

### Part V — Advanced & real-world (expert)

| # | Lesson | Example | You'll learn |
| --- | --- | --- | --- |
| 17 | [Quantum machine learning](lessons/17-quantum-ml.md) | `17_quantum_ml` | Quantum kernels (QSVM) and variational models |
| 18 | [Quantum error correction](lessons/18-error-correction.md) | `18_error_correction` | Steane/Shor codes, syndromes, logical qubits |
| 19 | [Noise & the NISQ reality](lessons/19-noise.md) | `19_noise` | Noise channels, device T1/T2, why circuits stay shallow |
| 20 | [BB84 quantum key distribution](lessons/20-bb84.md) | `20_bb84` | Detectable eavesdropping and quantum-secure keys |
| 21 | [Simulating noise (density matrix)](lessons/21-noise-simulation.md) | `21_noise_simulation` | Run circuits under noise; purity & fidelity |
| 22 | [Choosing a backend](lessons/22-backends.md) | `22_backends` | Run the same circuit on simulators vs an emulated QPU |
| 23 | [Asynchronous execution](lessons/23-async-jobs.md) | `23_async_jobs` | Submit jobs, poll to completion, target any backend |
| 24 | [Transpilation](lessons/24-transpilation.md) | `24_transpilation` | Decompose circuits to a native gate basis; the gate-count cost |
| 25 | [The Circuit Library](lessons/25-circuit-library.md) | `25_circuit_library` | The 53 building-block circuits, controlled rotations (`crx/cry/crz`), and the swap test |

## A one-minute concepts primer

- **Qubit** — a two-level quantum system. Its state is `α|0> + β|1>` with
  complex `α, β` and `|α|² + |β|² = 1`. `|α|²` is the probability of measuring 0.
- **Superposition** — being in a weighted combination of `|0>` and `|1>` at once.
- **Entanglement** — a joint state of multiple qubits that can't be described
  qubit-by-qubit; measuring one constrains the others.
- **Gate** — a reversible (unitary) operation on qubits. Single-qubit gates
  rotate the Bloch sphere; two-qubit gates like CNOT create entanglement.
- **Measurement** — reading a qubit collapses its superposition to 0 or 1 with
  the corresponding probability. You recover the distribution by repeating
  ("shots").

Ready? Start with [Lesson 1](lessons/01-hello-quantum.md).

## Related

- [casq-sdk](../casq-sdk) — the Rust client every lesson is built on.
- [casimirQ](../casimirQ) — the quantum simulation platform behind the SDK.
- [Ecosystem roadmap](../casimirQ/ROADMAP.md) — where the platform, SDK, and tutorial are headed.

## License

MIT
