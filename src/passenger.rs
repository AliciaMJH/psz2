pub(crate) enum PassengerState {
    Idle(u8),
    Entering,
    Choosing,
    InsideElevator(u8),
    Exiting,
}