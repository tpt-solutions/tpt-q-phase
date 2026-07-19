# TPT Q Phase — Project TODO

Open-source, memory-safe quantum control stack. Dual-licensed MIT / Apache-2.0, © TPT Solutions.

Tasks are grouped into phases. Phases 7 and 8 are hardware-dependent and blocked until lab hardware (FPGA dev boards, AWGs, dilution fridge access) is acquired — everything else can proceed against software simulators.

---

## Phase 0 — Project & Repo Setup
- [ ] Initialize git repository
- [ ] Add `.gitignore` (Rust/Cargo, VHDL/Verilog build artifacts, Go/Python if used)
- [ ] Add `LICENSE-MIT` (© TPT Solutions)
- [ ] Add `LICENSE-APACHE` (© TPT Solutions)
- [ ] Add dual-license notice to README ("Licensed under either of Apache License 2.0 or MIT license at your option")
- [ ] Establish SPDX header convention for source files (`SPDX-License-Identifier: MIT OR Apache-2.0`)
- [ ] Scaffold Cargo workspace (`Cargo.toml`) with member crates: sequencer, cryo-control, calibration, interface
- [ ] Set up CI skeleton (build + test on push/PR)
- [ ] Add `CONTRIBUTING.md`
- [ ] Add `CODE_OF_CONDUCT.md`
- [ ] Add issue and PR templates

## Phase 1 — Quantum Circuit Simulator & Core Types
*(no hardware needed)*
- [ ] Define core Rust types: qubit state representation
- [ ] Define gate set (Hadamard, CNOT, etc.)
- [ ] Define pulse parameter types (amplitude, frequency, phase, timing)
- [ ] Build or integrate a quantum circuit simulator backend (custom Rust simulator, or Qiskit/Cirq bindings) as a stand-in for real hardware
- [ ] Unit tests for gate sequences against the simulator

## Phase 2 — Pulse Sequencer (software model)
- [ ] Design deterministic scheduling model for nanosecond-precision pulse timing
- [ ] Implement `no_std` core logic translating gate sequences → pulse parameter streams
- [ ] Build software-only timing/jitter validation harness (stand-in for real clock hardware)
- [ ] Property/unit tests for determinism and timing bounds (<10ns jitter target)

## Phase 3 — Cryogenic Control Layer (software model)
- [ ] Define temperature-stage data model (300K / 4K / 100mK / 10mK)
- [ ] Define heater control interface as a trait/abstraction
- [ ] Implement a simulated cryostat backend for development without real hardware
- [ ] Implement PID/control-loop logic for heater adjustment
- [ ] Tests against simulated thermal profiles

## Phase 4 — Qubit Calibration Engine
- [ ] Define calibration data model: T1 relaxation, T2 coherence, gate fidelity
- [ ] Implement characterization routines against the Phase 1 simulator backend
- [ ] Implement auto-tuning feedback loop for pulse parameters
- [ ] Validation tests using simulated qubit noise models

## Phase 5 — Quantum-Classical Interface (API layer)
- [ ] **Open decision:** choose Go vs Python for the API layer
- [ ] Define API contract: submit circuit, retrieve results, streaming/polling for hybrid workflows
- [ ] Implement API server backed by the simulator/pulse-sequencer stack
- [ ] Build client SDK/example
- [ ] Integration tests

## Phase 6 — Error Correction Layer
- [ ] Implement surface code encoding/decoding logic
- [ ] Real-time syndrome monitoring against simulated qubit states
- [ ] Corrective pulse dispatch back through the Pulse Sequencer interface
- [ ] Tests using simulated decoherence/error injection

## Phase 7 — FPGA Integration
**Blocked: requires FPGA dev board**
- [ ] Define Rust → FPGA configuration generation interface (codegen boundary)
- [ ] VHDL/Verilog firmware skeleton for pulse generation
- [ ] Bring-up plan for first FPGA target once board is available

## Phase 8 — Physical Hardware Bring-Up
**Blocked: requires AWGs and dilution fridge access**
- [ ] AWG driver integration
- [ ] Real dilution fridge sensor/heater hardware interface (replacing Phase 3 simulator)
- [ ] End-to-end hardware validation against <10ns jitter target
- [ ] Safety interlocks

## Phase 9 — Hardening, Docs & Release
- [ ] Security/memory-safety audit pass
- [ ] Full architecture documentation
- [ ] API documentation
- [ ] Contributor guide
- [ ] Versioned pre-alpha/alpha release tagging
- [ ] Public announcement/launch checklist

---

## Notes
- TPT DataCenter, TPT Lithos, and TPT Fulcrum integrations are **out of scope** for this checklist — tracked only as external interface contracts consumed by Phase 5.
- Phases 7–8 stay blocked until lab hardware is acquired; all other phases can proceed fully in software/simulation.
