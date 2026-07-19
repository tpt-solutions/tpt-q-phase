// SPDX-License-Identifier: MIT OR Apache-2.0

//! PID control loop for heater adjustment.

use crate::state::{HeaterControl, Stage};

/// A discrete-time PID controller that adjusts heater power to hold a stage at
/// its target temperature.
#[derive(Clone, Debug)]
pub struct PidController {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    prev_error: f64,
}

impl PidController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    /// Compute the next heater power (clamped 0..=1) given the current
    /// temperature and target.
    pub fn update(&mut self, current: f64, target: f64, dt: f64) -> f64 {
        let error = target - current;
        self.integral += error * dt;
        // Anti-windup clamp.
        self.integral = self.integral.clamp(-10.0, 10.0);
        let derivative = if dt > 0.0 {
            (error - self.prev_error) / dt
        } else {
            0.0
        };
        self.prev_error = error;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        output.clamp(0.0, 1.0)
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
    }
}

/// Drive a heater with a PID loop for one step, returning the applied power.
pub fn regulate<H: HeaterControl>(
    pid: &mut PidController,
    heater: &mut H,
    stage: Stage,
    current: f64,
    target: f64,
    dt: f64,
) -> f64 {
    let power = pid.update(current, target, dt);
    heater.set_power(stage, power);
    power
}
