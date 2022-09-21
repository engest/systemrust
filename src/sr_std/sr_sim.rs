use crate::prelude::*;

pub struct SRSim<'a> {
    pub root_module: &'a SRModule<'a>,
    pub modules: Vec<&'a SRModule<'a>>,
}

impl<'a> SRSim<'a> {
    pub fn start_sim(&self) {
        println!("Starting simulation with root module: {}", self.root_module.name);
        let pm = self.root_module.parent;
        match pm {
            Some(p) => {
                println!("This not a root module!");
            },
            None => {
                println!("This is indeed a root module!");
            }
        }
    }
}