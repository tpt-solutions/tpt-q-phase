// SPDX-License-Identifier: MIT OR Apache-2.0

//! Surface-code error correction layer (software model).
//!
//! Encodes logical qubits with a rotated surface code, extracts stabilizers
//! (syndromes) from simulated qubit states with error injection, and decodes
//! using a minimum-weight perfect-matching-friendly lookup to dispatch
//! corrective pulses back through the sequencer interface.

use sequencer::gates::Gate;
use sequencer::pulse::Pulse;

/// Distance of the (rotated) surface code. Code uses `d x d` physical qubits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeDistance(pub usize);

impl CodeDistance {
    /// Number of physical qubits required for a rotated surface code of this
    /// distance.
    pub fn physical_qubits(&self) -> usize {
        self.0 * self.0
    }
}

/// A stabilizer measurement outcome: which syndrome qubits fired (1) vs not (0).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Syndrome {
    /// Fired stabilizer measurements, indexed by stabilizer id.
    pub fired: std::vec::Vec<bool>,
}

/// Encoded logical qubit state under a surface-code patch.
#[derive(Clone, Debug)]
pub struct SurfaceCode {
    distance: CodeDistance,
    /// Per-physical-qubit Pauli error currently applied (`X`, `Z`, or none).
    errors: std::vec::Vec<Pauli>,
}

/// Pauli operator on a single physical qubit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pauli {
    I,
    X,
    Z,
    Y,
}

impl Pauli {
    pub fn to_gate(self, q: usize) -> Option<Gate> {
        match self {
            Pauli::I => None,
            Pauli::X => Some(Gate::X(q)),
            Pauli::Z => Some(Gate::Z(q)),
            Pauli::Y => Some(Gate::Y(q)),
        }
    }
}

impl SurfaceCode {
    /// Create an empty `d x d` code patch with no errors.
    pub fn new(distance: CodeDistance) -> Self {
        Self {
            distance,
            errors: std::vec![Pauli::I; distance.physical_qubits()],
        }
    }

    pub fn distance(&self) -> CodeDistance {
        self.distance
    }

    /// Inject a Pauli error on a physical qubit (simulated decoherence).
    pub fn inject_error(&mut self, qubit: usize, err: Pauli) {
        if let Some(slot) = self.errors.get_mut(qubit) {
            *slot = err;
        }
    }

    /// Count of physical qubits currently carrying an error.
    pub fn error_count(&self) -> usize {
        self.errors.iter().filter(|e| **e != Pauli::I).count()
    }

    /// Extract a syndrome by measuring stabilizers. For the software model we
    /// use a simplified plaquette/star parity: a stabilizer fires when an odd
    /// number of adjacent data qubits carry a matching error.
    pub fn measure_syndrome(&self) -> Syndrome {
        let d = self.distance.0;
        let mut fired = std::vec::Vec::new();
        // Star (X-type) stabilizers on even plaquettes.
        for r in 0..(d - 1) {
            for c in 0..(d - 1) {
                let parity = self
                    .neighbors(r, c)
                    .iter()
                    .filter(|&&q| self.errors[q] == Pauli::X || self.errors[q] == Pauli::Y)
                    .count()
                    % 2;
                fired.push(parity == 1);
            }
        }
        Syndrome { fired }
    }

    fn neighbors(&self, r: usize, c: usize) -> std::vec::Vec<usize> {
        let d = self.distance.0;
        let mut out = std::vec::Vec::new();
        for (dr, dc) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            let rr = r + dr;
            let cc = c + dc;
            if rr < d && cc < d {
                out.push(rr * d + cc);
            }
        }
        out
    }

    /// Naive decoder: for each fired syndrome, flag the lowest-index adjacent
    /// qubit for an X correction. Returns the corrective gates to dispatch.
    pub fn decode_and_correct(&mut self, syndrome: &Syndrome) -> std::vec::Vec<Gate> {
        let mut corrections = std::vec::Vec::new();
        let d = self.distance.0;
        let mut idx = 0usize;
        for r in 0..(d - 1) {
            for c in 0..(d - 1) {
                if syndrome.fired.get(idx).copied().unwrap_or(false) {
                    let q = r * d + c;
                    // Flip the X component of the error to correct.
                    self.errors[q] = match self.errors[q] {
                        Pauli::X => Pauli::I,
                        Pauli::Y => Pauli::Z,
                        other => other,
                    };
                    if let Some(g) = Pauli::X.to_gate(q) {
                        corrections.push(g);
                    }
                }
                idx += 1;
            }
        }
        corrections
    }

    /// Translate a corrective gate list into sequencer pulses (Drag envelope).
    pub fn corrections_to_pulses(&self, gates: &[Gate], start_ns: i64) -> std::vec::Vec<Pulse> {
        gates
            .iter()
            .enumerate()
            .filter_map(|(i, g)| match g {
                Gate::X(q) | Gate::Y(q) | Gate::Z(q) => Some(Pulse {
                    channel: *q,
                    start_ns: start_ns + (i as i64) * 20,
                    duration_ns: 20,
                    amplitude: 1.0,
                    frequency_hz: 5.0e9,
                    phase: 0.0,
                    shape: sequencer::pulse::Envelope::Drag,
                }),
                _ => None,
            })
            .collect()
    }
}
