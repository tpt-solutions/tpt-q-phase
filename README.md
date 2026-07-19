# TPT Q Phase

Open-source, memory-safe quantum control stack.

TPT Q Phase is an open-source, memory-safe quantum control stack written in Rust.
It handles the deterministic, nanosecond-precision microwave pulses required to
control qubits, breaking the hardware lock-in of proprietary quantum control systems
and enabling a new generation of quantum-AI hybrid systems.

## Components

- **Pulse Sequencer** (`sequencer`): bare-metal, `no_std` deterministic scheduling of
  microwave pulses with nanosecond precision.
- **Cryogenic Control Layer** (`cryo-control`): temperature-stage monitoring and heater
  control for dilution-refrigerator operation.
- **Qubit Calibration Engine** (`calibration`): characterization of T1/T2/fidelity and
  auto-tuning of pulse parameters.
- **Quantum-Classical Interface** (`interface`): API layer for classical AI models to
  submit circuits and retrieve results.
- **Error Correction Layer** (`error-correction`): surface-code syndrome monitoring and
  corrective pulse dispatch.

## Status

Pre-alpha. Phases 1–6 are developed against software simulators. Phases 7–8
(FPGA integration, physical hardware bring-up) are blocked pending lab hardware.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

SPDX-License-Identifier: MIT OR Apache-2.0
