use std::time::Duration;
use std::thread;

fn is_the_car_in_place() -> bool {
    unimplemented!()
}
fn is_the_bolt_in_place() -> bool {
    unimplemented!()
}
fn move_arm_to_new_door() {
    unimplemented!();
}
fn move_arm_to_car() {
    unimplemented!()
}
fn turn_bolt() {
    unimplemented!()
}
fn grip_door() {
    unimplemented!()
}

struct WaitingCar;
struct TakingDoor;
struct PlacingDoor;

struct DoorMachine<S> {
    state: S,
}

impl From<DoorMachine<WaitingCar>> for DoorMachine<TakingDoor> {
    fn from(st: DoorMachine<WaitingCar>) -> DoorMachine<TakingDoor> {
        while !is_the_car_in_place() {
            thread::sleep(Duration::from_secs(1));
        }
        DoorMachine { state: TakingDoor }
    }
}

impl From<DoorMachine<TakingDoor>> for DoorMachine<PlacingDoor> {
    fn from(st: DoorMachine<TakingDoor>) -> DoorMachine<PlacingDoor> {
        move_arm_to_new_door();
        grip_door();

        DoorMachine { state: PlacingDoor }
    }
}

impl From<DoorMachine<PlacingDoor>> for DoorMachine<WaitingCar> {
    fn from(st: DoorMachine<PlacingDoor>) -> DoorMachine<WaitingCar> {
        move_arm_to_car();
        while !is_the_bolt_in_place() {
            turn_bolt();
        }

        DoorMachine { state: WaitingCar }
    }
}

// Will panic
fn main() {
    let beginning_state = DoorMachine { state: WaitingCar };
    let next_state: DoorMachine<TakingDoor> = beginning_state.into();

    // Lots of code

    let last_state: DoorMachine<PlacingDoor> = next_state.into();
}
