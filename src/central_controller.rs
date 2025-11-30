use crate::elevator::{Elevator, ElevatorFloorStatus};
use crate::floor_request::FloorRequest;
use::std::thread;
use::std::time::Duration;
use::std::sync::mpsc; // Multiple Producer, Single Consumer

pub struct CentralController {
    // Empfängt Inner Floor Request (eig. vom Passenger)
    floor_request_receiver: mpsc::Receiver<FloorRequest>,

    // Sender für jeden Elevator
    elevator_senders: Vec<mpsc::Sender<u8>>,

    // Empfängt Status (struct) vom Elevator (relevant für Türen)
    elevator_status_receiver: mpsc::Receiver<ElevatorFloorStatus>,

}

impl CentralController {
    pub fn new() -> (Self, mpsc::Sender<FloorRequest>) {
        // Erstellt Channel für FloorRequests -> CentralController
        let (floor_request_sender, floor_request_receiver) = mpsc::channel();
        // Erstellt Channel für ElevatorStatus -> CentralController
        let (elevator_status_sender, elevator_status_receiver) = mpsc::channel();

        let controller = Self {
            floor_request_receiver,
            elevator_senders: Vec::new(),
            elevator_status_receiver,
        };

        // Return Controller & Sender, damit andere den Sender nutzen können
        (controller, floor_request_sender)
    }


    pub fn start(&mut self) {
        println!("Central Controller starting elevators...");

        // Erstellt Channel ElevatorStatus -> CentralController
        let (elevator_status_sender, elevator_status_receiver) = mpsc::channel();
        self.elevator_status_receiver = elevator_status_receiver;

        let mut handles = Vec::new();

        for id in 1..4 {
            // Erstellt Channel CentralController -> Elevator (Sender in Vec, Receiver in Thread)
            let (sender_to_elevator, receiver_for_elevator) = mpsc::channel::<u8>();
            self.elevator_senders.push(sender_to_elevator);

            // Klont ElevatorStatus Sender für disen Thread
            // (müssen den Sender von hier an den Thread übergeben & können es nicht vom Elevator aus machen,
            // da wir sonst mehrere Receiver hier im Cotroller haben)
            let elevator_status_sender_clone = elevator_status_sender.clone();

            // Startet neuen Thread, erstellt eigenen Elevator
            let handle = thread::spawn(move || {
                run_elevator(id, receiver_for_elevator, elevator_status_sender_clone);
            });

            // Thread Handle speichern für später
            handles.push(handle);
        }

        println!("Central Controller listening for floor requests...");

        // Loop zum prüfen der Receiver für ElevatorFloorStatus & FloorRequest
        // Note: try_recv() blockiert nicht (anders als recv()), deshalb können  channels gelesen werden
        loop {

            if let Ok(status) = self.elevator_status_receiver.try_recv() {
                // TODO mit Türen in Verbindung bringen
                println!("Controller received status {:?}", status);
            }

            if let Ok(request) = self.floor_request_receiver.try_recv() {
                // TODO an mehr als nur einen ELevator weiterleiten
                println!("Controller received request {:?}", request);
                self.elevator_senders[0].send(request.floor()).unwrap();
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}

pub fn run_elevator(
    id: u8,
    receiver: mpsc::Receiver<u8>,
    elevator_status_sender: mpsc::Sender<ElevatorFloorStatus>,
) {
    let mut elevator = Elevator::new(id);

    // recv() blockiert & wartet auf Nachrichten
    loop {
        match receiver.recv() {
            // Request in Queue & verarbeiten
            Ok(target_floor) => {
                println!("Elevator {} received target floor: {}", id, target_floor);
                elevator.request_floor(target_floor);
                elevator.process_queue();

                // Sendet Status nach Verarbeitung der Queue
                let status = elevator.get_full_elevator_status();
                elevator_status_sender.send(status).unwrap();
            },
            Err(_) => {
                println!("Elevator {} shutting down...", id);
                break;
            }
        }
    }
}