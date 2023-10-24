mod stack;
mod memory;
mod runner;
use crate::runner::Runner;
use std::env;

fn main() {
    // take arg of bytecode, return error if no arg
    let bytecode: String = env::args().nth(1).expect("Plz enter bytecode as an arg");
    let mut runner = Runner::from_code(bytecode);
    // runner, execution 
}