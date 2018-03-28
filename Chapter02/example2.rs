pub trait StateMachine {
    type Next: MainLogic;
    fn execute(self) -> Self::Next;
}

pub trait MainLogic {
    fn main_logic(self);
}

impl<S> MainLogic for S
where
    S: StateMachine,
{
    fn main_logic(self) {
        self.execute().main_logic();
    }
}

struct FirstState;
struct LastState;

impl StateMachine for FirstState {
    type Next = LastState;

    fn execute(self) -> Self::Next {
        unimplemented!()
    }
}

impl MainLogic for LastState {
    fn main_logic(self) {
        unimplemented!()
    }
}

// Will panic
fn main() {
    let start = FirstState;
    start.execute().main_logic();
}
