#![no_std]

use sails_rs::{gstd::msg, prelude::*};
use gstd::exec;

struct DelayedMessageService(());

#[sails_rs::service]
impl DelayedMessageService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn send_delayed_message(&mut self) {
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            [],
            0,
            0,
            10,
        )
        .expect("Error in sending message");
    }
}

pub struct DelayedMessageProgram(());

#[sails_rs::program]
impl DelayedMessageProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn delayed_message(&self) -> DelayedMessageService {
        DelayedMessageService::new()
    }
}
