mod elevator;
mod door;
mod passenger;
mod central_controller;
mod floor_request;
use std::thread;
use std::time::Duration;
use central_controller::CentralController;
use floor_request::FloorRequest;
use crate::floor_request::Direction;

fn main() {
    let (mut controller, sender) = CentralController::new();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        sender.send(FloorRequest::new(2, Direction::Up)).unwrap();

        thread::sleep(Duration::from_secs(2));
        sender.send(FloorRequest::new(1, Direction::Down)).unwrap();
    });

    controller.start();

}
