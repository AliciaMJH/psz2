use crate::elevator::ElevatorState::{Idle, Moving, Stopping};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ElevatorState {
    Idle(u8),
    Moving { from: u8, to: u8 },
    Stopping(u8),
}

// Public struct, um Status an Controller zu senden
#[derive(Debug, Clone, Copy)]
pub(crate) struct ElevatorFloorStatus {
    pub(crate) elevator_id: u8,
    pub(crate) current_floor: u8,
    pub(crate) elevator_state: ElevatorState,
}

pub(crate) struct Elevator {
    // Felder sind private
    id: u8,
    current_floor: u8,
    target_floors: VecDeque<u8>, //Queue
    elevator_state: ElevatorState,
}

// Methoden Implementierung
impl Elevator {
    // Erstellt neuen Elevator mit vorgegebener id
    pub(crate) fn new(id: u8) -> Self {
        assert!(id < 4, "Elevator id out of range! Only ids 1-3 allowed!");
        Self {
            id,
            current_floor: 0,
            target_floors: VecDeque::new(),
            elevator_state: Idle(0),
        }
    }

    // Getter für id
    pub(crate) fn id(&self) -> u8 { self.id }
    // Getter für current_floor
    pub(crate) fn current_floor(&self) -> u8 { self.current_floor }
    // Getter für gesamten Status (id, floor, state), mappen in ElevatorFloorStatus
    pub(crate) fn get_full_elevator_status(&self) -> ElevatorFloorStatus {
        ElevatorFloorStatus {
            elevator_id: self.id,
            current_floor: self.current_floor,
            elevator_state: self.elevator_state,
        }
    }

    // Bewegt Elevator um 1 nach oben - private
    fn move_up(&mut self) {
        assert!(self.current_floor < 3, "Elevator {} can not go up higher!", self.id);
        self.current_floor += 1;
        println!("Elevator {} on level {}", self.id, self.current_floor);
    }

    // Bewegt Elevator um 1 nach unten - private
    fn move_down(&mut self) {
        assert!(self.current_floor > 0, "Elevator {} can not go down lower!", self.id);
        self.current_floor -= 1;
        println!("Elevator {} on level {}", self.id, self.current_floor);
    }

    // Bewegt Elevator zu gewünschtem Stockwerk unter Nutzung von move_up & move_down
    // (damit inkrementell Stockwerke angezeigt werden)
    pub(crate) fn move_to(&mut self) {
        // Hole target vorne aus Queue
        let target = match self.target_floors.pop_front() {
            Some(floor) => floor,
            None => {
                println!("No targets in queue for elevator {}", self.id);
                return;
            },
        };

        // Falls Elevator bereits auf target, do nothing
        if self.current_floor == target {
            println!("Elevator {} already on level {}", self.id, target);
            return;
        }

        // Setze elevator_state auf Moving
        self.elevator_state = Moving {from: self.current_floor, to: target};
        println!("Elevator {} {:?}", self.id, self.elevator_state);

        // Höheres Stockwerk noch nicht erreicht
        while self.current_floor < target {
            self.move_up();
        }
        // Tieferes Stockwerk noch nicht erreicht
        while self.current_floor > target {
            self.move_down();
        }

        // Setze elevator_state auf Stopping im Zielstockwerk
        self.elevator_state = Stopping(self.current_floor);
        println!("Elevator {} stopped at level {}", self.id, self.current_floor);

        // Setze elevator_state auf Idle, remove target_floor
        self.elevator_state = Idle(self.current_floor);
    }

    // Repräsentiert Fahrstuhlknöpfe INNERHALB der Kabine
    pub(crate) fn request_floor(&mut self, floor: u8) {
        assert!(floor < 4, "Invalid floor!");

        // Wenn Zielstockwerk noch nicht in Queue, add hinten
        if !self.target_floors.contains(&floor) {
            self.target_floors.push_back(floor);
            println!("Elevator {} received request to go to floor {}", self.id, floor);
        }
    }

    pub(crate) fn process_queue(&mut self) {
        while !self.target_floors.is_empty() {
            self.move_to();
        }
        println!("Elevator {} finished processing all requests!", self.id);
    }
}