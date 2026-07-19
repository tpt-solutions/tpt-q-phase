// SPDX-License-Identifier: MIT OR Apache-2.0

//! State-vector quantum circuit simulator.
//!
//! A lightweight, dependency-free simulator used as a stand-in for real
//! hardware while the stack is developed in software. It applies single- and
//! two-qubit gates to a [`QubitState`] and supports measurement sampling.

use crate::gates::{Gate, SingleQubitGate, TwoQubitGate};
use crate::state::QubitState;

/// Errors raised by the simulator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimError {
    /// A gate references a qubit index outside the register.
    QubitOutOfRange,
}

/// A quantum circuit simulator over `n` qubits.
#[derive(Clone, Debug)]
pub struct Simulator {
    state: QubitState,
    /// Deterministic RNG state (xorshift32) for measurement sampling.
    rng_state: u32,
}

impl Simulator {
    /// Create a simulator initialized to the all-zero state on `num_qubits`.
    pub fn new(num_qubits: usize) -> Self {
        Self {
            state: QubitState::zero(num_qubits),
            rng_state: 0x9E3779B9,
        }
    }

    /// Number of qubits.
    pub fn num_qubits(&self) -> usize {
        self.state.num_qubits()
    }

    /// Borrow the current state vector.
    pub fn state(&self) -> &QubitState {
        &self.state
    }

    /// Apply a single-qubit gate to qubit `q`.
    pub fn apply_single(&mut self, gate: SingleQubitGate, q: usize) -> Result<(), SimError> {
        if q >= self.state.num_qubits() {
            return Err(SimError::QubitOutOfRange);
        }
        let [a, b] = gate.matrix;
        let amps = self.state.amplitudes_mut();
        let n = amps.len();
        let step = 1usize << q;
        for base in 0..n {
            if (base & step) != 0 {
                continue;
            }
            let i0 = base;
            let i1 = base | step;
            let v0 = amps[i0];
            let v1 = amps[i1];
            amps[i0] = a[0] * v0 + a[1] * v1;
            amps[i1] = b[0] * v0 + b[1] * v1;
        }
        Ok(())
    }

    /// Apply a two-qubit (control, target) gate.
    pub fn apply_two(
        &mut self,
        gate: TwoQubitGate,
        control: usize,
        target: usize,
    ) -> Result<(), SimError> {
        let nq = self.state.num_qubits();
        if control >= nq || target >= nq || control == target {
            return Err(SimError::QubitOutOfRange);
        }
        let m = gate.matrix;
        let amps = self.state.amplitudes_mut();
        let n = amps.len();
        let c = 1usize << control;
        let t = 1usize << target;
        // Four amplitude subspaces distinguished by (control, target) bits.
        for base in 0..n {
            if (base & (c | t)) != 0 {
                continue;
            }
            let i00 = base;
            let i01 = base | t;
            let i10 = base | c;
            let i11 = base | c | t;
            let v00 = amps[i00];
            let v01 = amps[i01];
            let v10 = amps[i10];
            let v11 = amps[i11];
            for (row, idx) in [(0usize, i00), (1, i01), (2, i10), (3, i11)] {
                let acc = m[row][0] * v00 + m[row][1] * v01 + m[row][2] * v10 + m[row][3] * v11;
                amps[idx] = acc;
            }
        }
        Ok(())
    }

    /// Apply a gate from the standard gate set.
    pub fn apply(&mut self, gate: Gate) -> Result<(), SimError> {
        match gate {
            Gate::H(q) => self.apply_single(SingleQubitGate::H, q),
            Gate::X(q) => self.apply_single(SingleQubitGate::X, q),
            Gate::Y(q) => self.apply_single(SingleQubitGate::Y, q),
            Gate::Z(q) => self.apply_single(SingleQubitGate::Z, q),
            Gate::S(q) => self.apply_single(SingleQubitGate::S, q),
            Gate::T(q) => self.apply_single(SingleQubitGate::T, q),
            Gate::Rx(q) => self.apply_single(SingleQubitGate::rx(0.0), q),
            Gate::Ry(q) => self.apply_single(SingleQubitGate::ry(0.0), q),
            Gate::Rz(q) => self.apply_single(SingleQubitGate::rz(0.0), q),
            Gate::CNOT(c, t) => self.apply_two(TwoQubitGate::CNOT, c, t),
            Gate::CZ(c, t) => self.apply_two(TwoQubitGate::CZ, c, t),
        }
    }

    /// Measure all qubits, returning an integer whose bit `q` is the result of
    /// qubit `q`. Uses the simulator's internal deterministic RNG.
    pub fn measure_all(&mut self) -> u64 {
        let mut outcome = 0u64;
        let r = self.next_u32() as f32 / (u32::MAX as f32);
        let mut cumulative = 0.0f32;
        let n = self.state.dim();
        for i in 0..n {
            cumulative += self.state.probability(i);
            if r <= cumulative {
                outcome = i as u64;
                break;
            }
        }
        outcome
    }

    /// Sample `shots` full measurements, returning a histogram keyed by outcome.
    pub fn sample(&mut self, shots: usize) -> alloc::vec::Vec<(u64, usize)> {
        let mut counts = alloc::collections::BTreeMap::<u64, usize>::new();
        for _ in 0..shots {
            *counts.entry(self.measure_all()).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_state = x;
        x
    }
}
