# TPT Q Phase — Project TODO

Open-source, memory-safe quantum control stack. Dual-licensed MIT / Apache-2.0, © TPT Solutions.

Tasks are grouped into phases. Phases 7 and 8 are hardware-dependent and blocked until lab hardware (FPGA dev boards, AWGs, dilution fridge access) is acquired — everything else can proceed against software simulators.

---

## Phase 0 — Project & Repo Setup
- [x] Initialize git repository
- [x] Add `.gitignore` (Rust/Cargo, VHDL/Verilog build artifacts, Go/Python if used)
- [x] Add `LICENSE-MIT` (© TPT Solutions)
- [x] Add `LICENSE-APACHE` (© TPT Solutions)
- [x] Add dual-license notice to README ("Licensed under either of Apache License 2.0 or MIT license at your option")
- [x] Establish SPDX header convention for source files (`SPDX-License-Identifier: MIT OR Apache-2.0`)
- [x] Scaffold Cargo workspace (`Cargo.toml`) with member crates: sequencer, cryo-control, calibration, interface
- [x] Set up CI skeleton (build + test on push/PR)
- [x] Add `CONTRIBUTING.md`
- [x] Add `CODE_OF_CONDUCT.md`
- [x] Add issue and PR templates

## Phase 1 — Quantum Circuit Simulator & Core Types
*(no hardware needed)*
- [x] Define core Rust types: qubit state representation
- [x] Define gate set (Hadamard, CNOT, etc.)
- [x] Define pulse parameter types (amplitude, frequency, phase, timing)
- [x] Build or integrate a quantum circuit simulator backend (custom Rust simulator, or Qiskit/Cirq bindings) as a stand-in for real hardware
- [x] Unit tests for gate sequences against the simulator

## Phase 2 — Pulse Sequencer (software model)
- [x] Design deterministic scheduling model for nanosecond-precision pulse timing
- [x] Implement `no_std` core logic translating gate sequences → pulse parameter streams
- [x] Build software-only timing/jitter validation harness (stand-in for real clock hardware)
- [x] Property/unit tests for determinism and timing bounds (<10ns jitter target)

## Phase 3 — Cryogenic Control Layer (software model)
- [x] Define temperature-stage data model (300K / 4K / 100mK / 10mK)
- [x] Define heater control interface as a trait/abstraction
- [x] Implement a simulated cryostat backend for development without real hardware
- [x] Implement PID/control-loop logic for heater adjustment
- [x] Tests against simulated thermal profiles

## Phase 4 — Qubit Calibration Engine
- [x] Define calibration data model: T1 relaxation, T2 coherence, gate fidelity
- [x] Implement characterization routines against the Phase 1 simulator backend
- [x] Implement auto-tuning feedback loop for pulse parameters
- [x] Validation tests using simulated qubit noise models

## Phase 5 — Quantum-Classical Interface (API layer)
- [x] **Open decision:** choose Go vs Python for the API layer *(deferred — implemented contract in Rust as reference; see note below)*
- [x] Define API contract: submit circuit, retrieve results, streaming/polling for hybrid workflows
- [x] Implement API server backed by the simulator/pulse-sequencer stack
- [x] Build client SDK/example
- [x] Integration tests

> **Phase 5 API language note:** The Go-vs-Python decision remains open. The
> `interface` crate implements the language-neutral JSON API *contract* plus a
> simulator-backed reference server in Rust, so the contract is validated today
> and a Go/Python server can mirror it exactly later.

## Phase 6 — Error Correction Layer
- [x] Implement surface code encoding/decoding logic
- [x] Real-time syndrome monitoring against simulated qubit states
- [x] Corrective pulse dispatch back through the Pulse Sequencer interface
- [x] Tests using simulated decoherence/error injection

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
