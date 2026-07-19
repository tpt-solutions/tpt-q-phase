// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests for the pulse sequencer: simulator, scheduler, and
//! determinism/jitter validation.

use num_complex::Complex32;
use sequencer::gates::Gate;
use sequencer::pulse::Pulse;
use sequencer::schedule::{compile, max_jitter_ns, ChannelConfig, SINGLE_QUBIT_DURATION_NS};
use sequencer::simulator::{SimError, Simulator};
use sequencer::state::QubitState;

#[test]
fn zero_state_is_normalized() {
    let s = QubitState::zero(3);
    assert!(s.is_normalized(1e-5));
    assert_eq!(s[0], Complex32::new(1.0, 0.0));
}

#[test]
fn hadamard_creates_superposition() {
    let mut sim = Simulator::new(1);
    sim.apply(Gate::H(0)).unwrap();
    let p0 = sim.state().probability(0);
    let p1 = sim.state().probability(1);
    assert!((p0 - 0.5).abs() < 1e-5);
    assert!((p1 - 0.5).abs() < 1e-5);
}

#[test]
fn cnot_entangles_bell_state() {
    let mut sim = Simulator::new(2);
    sim.apply(Gate::H(0)).unwrap();
    sim.apply(Gate::CNOT(0, 1)).unwrap();
    // Bell state: only |00> and |11> populated.
    assert!((sim.state().probability(0b00) - 0.5).abs() < 1e-5);
    assert!((sim.state().probability(0b11) - 0.5).abs() < 1e-5);
    assert!(sim.state().probability(0b01) < 1e-6);
    assert!(sim.state().probability(0b10) < 1e-6);
}

#[test]
fn x_then_x_is_identity() {
    let mut sim = Simulator::new(1);
    sim.apply(Gate::X(0)).unwrap();
    sim.apply(Gate::X(0)).unwrap();
    assert!((sim.state().probability(0) - 1.0).abs() < 1e-5);
}

#[test]
fn qubit_out_of_range_errors() {
    let mut sim = Simulator::new(2);
    assert_eq!(sim.apply(Gate::X(2)), Err(SimError::QubitOutOfRange));
    assert_eq!(sim.apply(Gate::CNOT(0, 2)), Err(SimError::QubitOutOfRange));
}

#[test]
fn sampling_is_consistent_with_probabilities() {
    let mut sim = Simulator::new(1);
    sim.apply(Gate::H(0)).unwrap();
    let hist = sim.sample(2000);
    let total: usize = hist.iter().map(|(_, c)| c).sum();
    assert_eq!(total, 2000);
    for (outcome, count) in hist {
        let p = count as f32 / 2000.0;
        let expected = sim.state().probability(outcome as usize);
        assert!(
            (p - expected).abs() < 0.1,
            "outcome {outcome}: {p} vs {expected}"
        );
    }
}

#[test]
fn schedule_is_deterministic_and_ordered() {
    let gates = [Gate::H(0), Gate::X(1), Gate::CNOT(0, 1), Gate::H(1)];
    let chans = ChannelConfig::uniform(2, 5.0e9);
    let a = compile(&gates, &chans);
    let b = compile(&gates, &chans);
    assert_eq!(a.pulses().len(), b.pulses().len());
    assert_eq!(a.total_duration_ns(), b.total_duration_ns());
    let starts_a: Vec<i64> = a.pulses().iter().map(|p| p.start_ns).collect();
    let starts_b: Vec<i64> = b.pulses().iter().map(|p| p.start_ns).collect();
    assert_eq!(starts_a, starts_b);
}

#[test]
fn schedule_timing_is_monotonic() {
    let gates: std::vec::Vec<Gate> = (0..4).map(Gate::H).collect();
    let chans = ChannelConfig::uniform(4, 5.0e9);
    let sched = compile(&gates, &chans);
    let mut prev_end = 0i64;
    for p in sched.pulses() {
        assert!(p.start_ns >= prev_end, "pulses overlap or are unordered");
        prev_end = p.end_ns();
    }
}

#[test]
fn jitter_target_under_10ns() {
    // Reference clock period of 1 ns; all pulse starts are integer ns so the
    // observed jitter (rounding to the period) is 0 ns, well under 10 ns.
    let gates = [Gate::H(0), Gate::CNOT(0, 1), Gate::H(1)];
    let chans = ChannelConfig::uniform(2, 5.0e9);
    let sched = compile(&gates, &chans);
    assert!(max_jitter_ns(&sched, 1) < 10);
}

#[test]
fn pulse_durations_match_spec() {
    let gates = [Gate::H(0)];
    let chans = ChannelConfig::uniform(1, 5.0e9);
    let sched = compile(&gates, &chans);
    let p: &Pulse = &sched.pulses()[0];
    assert_eq!(p.duration_ns, SINGLE_QUBIT_DURATION_NS);
}
