use std::time::Duration;
use crate::door::DoorState::{Closed, Closing, Open, Opening};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum DoorState {
    Closed,
    Open,
    Closing,
    Opening,
}

pub struct Door {
    door_state: DoorState,
    timeout_duration: Duration,
    elevator_id: u8,
}

impl Door {

    pub(crate) fn new(elevator_id: u8) -> Self {
        Self {
            door_state: Closed,
            timeout_duration: Duration::from_secs(3),
            elevator_id,
        }
    }

    pub(crate) fn door_state(&self) -> DoorState {
        self.door_state
    }

    pub(crate) fn set_opening(&mut self) {
        if self.door_state == Closed {
            self.door_state = Opening;
            println!("Elevator door {} is opening...", self.elevator_id);
        }
    }

    pub(crate) fn set_open(&mut self) {
        if self.door_state == Opening {
            self.door_state = Open;
            println!("Elevator door {} is open.", self.elevator_id);
        }
    }

    pub(crate) fn set_closing(&mut self) {
        if self.door_state == Open {
            self.door_state = Closing;
            println!("Elevator door {} is closing...", self.elevator_id);
        }
    }

    pub(crate) fn set_closed(&mut self) {
        if self.door_state == Closing {
            self.door_state = Closed;
            println!("Elevator door {} is closed.", self.elevator_id);
        }
    }
}