// SPDX-License-Identifier: MIT OR Apache-2.0

//! Quantum-classical interface (API layer, software model).
//!
//! **Open decision (Phase 5):** the production API server is planned in Go or
//! Python. This crate defines the language-neutral API *contract* in Rust and
//! provides a simulator-backed reference implementation plus a client example,
//! so the contract can be exercised and validated today. The wire format is
//! JSON, matching what a Go/Python server would expose.

use serde::{Deserialize, Serialize};

/// A submitted quantum circuit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitRequest {
    /// Number of qubits.
    pub num_qubits: usize,
    /// Ordered gate sequence, serialized as named gates.
    pub gates: std::vec::Vec<GateSpec>,
    /// Number of measurement shots.
    pub shots: usize,
}

/// Serializable gate specification mirroring [`sequencer::gates::Gate`].
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GateSpec {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    T(usize),
    CNOT(usize, usize),
    CZ(usize, usize),
}

/// Result of executing a circuit: an outcome histogram.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitResult {
    /// Outcome histograms: (basis-state integer, count).
    pub counts: std::vec::Vec<(u64, usize)>,
    /// Wall-clock execution estimate, nanoseconds (simulation timing).
    pub elapsed_ns: u64,
}

/// Submit/result envelope for the streaming/polling workflow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiMessage {
    Submit(CircuitRequest),
    Result(CircuitResult),
}

/// Convert a [`GateSpec`] to the internal gate representation.
pub fn to_internal_gate(g: GateSpec) -> sequencer::gates::Gate {
    match g {
        GateSpec::H(q) => sequencer::gates::Gate::H(q),
        GateSpec::X(q) => sequencer::gates::Gate::X(q),
        GateSpec::Y(q) => sequencer::gates::Gate::Y(q),
        GateSpec::Z(q) => sequencer::gates::Gate::Z(q),
        GateSpec::S(q) => sequencer::gates::Gate::S(q),
        GateSpec::T(q) => sequencer::gates::Gate::T(q),
        GateSpec::CNOT(c, t) => sequencer::gates::Gate::CNOT(c, t),
        GateSpec::CZ(c, t) => sequencer::gates::Gate::CZ(c, t),
    }
}

/// Execute a circuit request against the simulator backend.
pub fn execute(req: &CircuitRequest) -> CircuitResult {
    let gates: std::vec::Vec<_> = req.gates.iter().copied().map(to_internal_gate).collect();
    let mut sim = sequencer::simulator::Simulator::new(req.num_qubits);
    for g in &gates {
        let _ = sim.apply(*g);
    }
    let counts = sim.sample(req.shots);
    CircuitResult {
        counts,
        elapsed_ns: (gates.len() as u64) * 20,
    }
}

/// Serialize a request to JSON (wire format for the future Go/Python server).
pub fn serialize_request(req: &CircuitRequest) -> Result<std::string::String, serde_json::Error> {
    serde_json::to_string(req)
}

/// Parse a JSON request (used by the server side).
pub fn parse_request(json: &str) -> Result<CircuitRequest, serde_json::Error> {
    serde_json::from_str(json)
}
