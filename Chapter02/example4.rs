pub trait PageTable {}

pub enum P4 {}
pub enum P3 {}
pub enum P2 {}
pub enum P1 {}

impl PageTable for P4 {}
impl PageTable for P3 {}
impl PageTable for P2 {}
impl PageTable for P1 {}

pub trait HighTable: PageTable {
    type NextTable: PageTable;
}

impl HighTable for P4 {
    type NextTable = P3;
}

impl HighTable for P3 {
    type NextTable = P2;
}

impl HighTable for P2 {
    type NextTable = P1;
}

// Dummy main function to check compilation.
fn main() {}
