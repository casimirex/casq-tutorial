# Lesson 12 — Quantum teleportation

**Example:** [`examples/12_teleportation.rs`](../examples/12_teleportation.rs) ·
`cargo run --example 12_teleportation`

## The idea

Quantum teleportation moves the *state* of a qubit from a sender ("Alice") to a
receiver ("Bob") without physically sending the qubit — using a shared entangled
pair plus **two classical bits**.

It has to work this indirect way because of two hard limits:

- **No-cloning theorem:** you can't copy an unknown quantum state. So teleport
  moves it; the original is unavoidably destroyed in the process.
- You can't just "measure and resend": measuring an unknown state collapses it
  and loses information.

The protocol:

1. Alice and Bob pre-share a Bell pair (one qubit each).
2. Alice interacts her unknown qubit with her half of the pair and measures both,
   getting two classical bits.
3. She sends those two bits to Bob over an ordinary channel.
4. Bob applies one of four corrections (`I`, `X`, `Z`, or `ZX`) based on the
   bits, and his qubit is now in Alice's original state.

No information travels faster than light — Bob's qubit is useless until the two
classical bits arrive.

## The code

casq-sdk runs the whole protocol through the algorithms API. You give the state
to send as amplitudes `(α, β)`, and it reports Bob's measurement probabilities
and the **fidelity** between sent and received states:

```rust
let r = client.algorithms().teleport(alpha, beta).await?;
println!("fidelity {:.4}", r.fidelity);
```

## What you'll see

```
  state             P(0)    P(1)    fidelity   verified
  |0>               1.000   0.000   1.0000     true
  |+>               0.500   0.500   1.0000     true
  0.6|0>+0.8|1>     0.360   0.640   1.0000     true
```

Fidelity `1.0` means Bob's qubit exactly matches the state Alice sent
(`P(1) = |β|² = 0.8² = 0.64`, as expected).

## Try it yourself

1. Send `(0.8, 0.6)` and predict `P(1)` before running.
2. Teleportation transfers *one* qubit's state using *one* Bell pair and *two*
   classical bits. What does that ratio tell you about entanglement as a
   resource?
3. Contrast with *superdense coding*, the mirror-image protocol that sends two
   classical bits using one qubit.

## Key takeaway

Entanglement + classical communication can relocate quantum information exactly,
within the rules set by no-cloning and no faster-than-light signalling.

Next: [Lesson 13 — VQE](13-vqe.md)
