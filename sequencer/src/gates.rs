// SPDX-License-Identifier: MIT OR Apache-2.0

//! Standard quantum gate set.
//!
//! Gates are represented as the underlying 2x2 (single-qubit) or 4x4
//! (two-qubit) unitary matrices acting on the state vector, plus their
//! decomposition into control pulses for the sequencer.

use num_complex::Complex32;

/// A single-qubit unitary gate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SingleQubitGate {
    pub matrix: [[Complex32; 2]; 2],
}

/// A two-qubit controlled unitary gate (e.g. CNOT, CZ).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoQubitGate {
    /// 4x4 matrix in big-endian (control, target) basis ordering.
    pub matrix: [[Complex32; 4]; 4],
}

/// Identifier for a gate in the standard circuit gate set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    T(usize),
    Rx(usize),
    Ry(usize),
    Rz(usize),
    CNOT(usize, usize),
    CZ(usize, usize),
}

const I: Complex32 = Complex32::new(0.0, 1.0);
const NEG_I: Complex32 = Complex32::new(0.0, -1.0);
const ONE: Complex32 = Complex32::new(1.0, 0.0);
const NEG_ONE: Complex32 = Complex32::new(-1.0, 0.0);
const ZERO: Complex32 = Complex32::new(0.0, 0.0);
const SQRT1_2: Complex32 = Complex32::new(core::f32::consts::FRAC_1_SQRT_2, 0.0);
const NEG_SQRT1_2: Complex32 = Complex32::new(-core::f32::consts::FRAC_1_SQRT_2, 0.0);

impl SingleQubitGate {
    pub const H: SingleQubitGate = SingleQubitGate {
        matrix: [[SQRT1_2, SQRT1_2], [SQRT1_2, NEG_SQRT1_2]],
    };
    pub const X: SingleQubitGate = SingleQubitGate {
        matrix: [[ZERO, ONE], [ONE, ZERO]],
    };
    pub const Y: SingleQubitGate = SingleQubitGate {
        matrix: [[ZERO, NEG_I], [I, ZERO]],
    };
    pub const Z: SingleQubitGate = SingleQubitGate {
        matrix: [[ONE, ZERO], [ZERO, NEG_ONE]],
    };
    pub const S: SingleQubitGate = SingleQubitGate {
        matrix: [[ONE, ZERO], [ZERO, I]],
    };
    pub const T: SingleQubitGate = SingleQubitGate {
        matrix: [
            [ONE, ZERO],
            [
                ZERO,
                Complex32::new(
                    core::f32::consts::FRAC_1_SQRT_2,
                    core::f32::consts::FRAC_1_SQRT_2,
                ),
            ],
        ],
    };

    /// Rotation about the X axis by `theta` radians.
    pub fn rx(theta: f32) -> Self {
        let c = (theta / 2.0).cos();
        let s = (theta / 2.0).sin();
        SingleQubitGate {
            matrix: [
                [Complex32::new(c, 0.0), Complex32::new(0.0, -s)],
                [Complex32::new(0.0, -s), Complex32::new(c, 0.0)],
            ],
        }
    }

    /// Rotation about the Y axis by `theta` radians.
    pub fn ry(theta: f32) -> Self {
        let c = (theta / 2.0).cos();
        let s = (theta / 2.0).sin();
        SingleQubitGate {
            matrix: [
                [Complex32::new(c, 0.0), Complex32::new(-s, 0.0)],
                [Complex32::new(s, 0.0), Complex32::new(c, 0.0)],
            ],
        }
    }

    /// Rotation about the Z axis by `theta` radians.
    pub fn rz(theta: f32) -> Self {
        let h = theta / 2.0;
        SingleQubitGate {
            matrix: [
                [Complex32::new(h.cos(), h.sin()), ZERO],
                [ZERO, Complex32::new(h.cos(), -h.sin())],
            ],
        }
    }

    pub fn from_gate(g: Gate) -> SingleQubitGate {
        match g {
            Gate::H(q) | Gate::X(q) | Gate::Y(q) | Gate::Z(q) | Gate::S(q) | Gate::T(q) => {
                SingleQubitGate::from_gate_on_qubit(g, q)
            }
            Gate::Rx(_) => SingleQubitGate::rx(0.0),
            Gate::Ry(_) => SingleQubitGate::ry(0.0),
            Gate::Rz(_) => SingleQubitGate::rz(0.0),
            _ => SingleQubitGate::X,
        }
    }

    fn from_gate_on_qubit(g: Gate, _q: usize) -> SingleQubitGate {
        match g {
            Gate::H(_) => SingleQubitGate::H,
            Gate::X(_) => SingleQubitGate::X,
            Gate::Y(_) => SingleQubitGate::Y,
            Gate::Z(_) => SingleQubitGate::Z,
            Gate::S(_) => SingleQubitGate::S,
            Gate::T(_) => SingleQubitGate::T,
            _ => SingleQubitGate::X,
        }
    }
}

impl TwoQubitGate {
    pub const CNOT: TwoQubitGate = TwoQubitGate {
        matrix: [
            [ONE, ZERO, ZERO, ZERO],
            [ZERO, ONE, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ONE],
            [ZERO, ZERO, ONE, ZERO],
        ],
    };
    pub const CZ: TwoQubitGate = TwoQubitGate {
        matrix: [
            [ONE, ZERO, ZERO, ZERO],
            [ZERO, ONE, ZERO, ZERO],
            [ZERO, ZERO, ONE, ZERO],
            [ZERO, ZERO, ZERO, Complex32::new(-1.0, 0.0)],
        ],
    };
}
