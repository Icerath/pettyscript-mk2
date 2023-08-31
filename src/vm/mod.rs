use crate::prelude::*;

pub struct Vm {
    instructions: Vec<Instruction>,
}

impl Vm {
    pub fn raise_not_implemented(&mut self) {
        println!("Not Implemented");
        unimplemented!()
    }
}
