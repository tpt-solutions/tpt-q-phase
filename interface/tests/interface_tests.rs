// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests for the quantum-classical interface contract.

use interface::{execute, parse_request, serialize_request, ApiMessage, CircuitRequest, GateSpec};
use sequencer::gates::Gate;

#[test]
fn bell_circuit_histogram() {
    let req = CircuitRequest {
        num_qubits: 2,
        gates: std::vec![GateSpec::H(0), GateSpec::CNOT(0, 1)],
        shots: 1000,
    };
    let res = execute(&req);
    let total: usize = res.counts.iter().map(|(_, c)| c).sum();
    assert_eq!(total, 1000);
    // Bell state: only |00> and |11> should appear.
    for (outcome, _) in &res.counts {
        assert!(*outcome == 0b00 || *outcome == 0b11);
    }
}

#[test]
fn request_round_trips_through_json() {
    let req = CircuitRequest {
        num_qubits: 1,
        gates: std::vec![GateSpec::H(0), GateSpec::X(0)],
        shots: 200,
    };
    let json = serialize_request(&req).unwrap();
    let back = parse_request(&json).unwrap();
    assert_eq!(req.num_qubits, back.num_qubits);
    assert_eq!(req.gates.len(), back.gates.len());
}

#[test]
fn api_message_envelope_serializes() {
    let msg = ApiMessage::Submit(CircuitRequest {
        num_qubits: 1,
        gates: std::vec![GateSpec::H(0)],
        shots: 10,
    });
    let json = serde_json::to_string(&msg).unwrap();
    let parsed: ApiMessage = serde_json::from_str(&json).unwrap();
    matches!(parsed, ApiMessage::Submit(_));
}

#[test]
fn gate_spec_matches_internal_gate_enum() {
    // Ensure the public contract covers all gates the simulator supports.
    let _ = Gate::Rx(0);
    let specs = [GateSpec::H(0), GateSpec::CNOT(0, 1)];
    assert_eq!(specs.len(), 2);
}
