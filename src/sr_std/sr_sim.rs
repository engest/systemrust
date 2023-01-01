use crate::prelude::*;

pub struct SRSim<'a> {
    pub root_module: &'a Box<SRModule>,
    // pub modules: Vec<Box<SRModule>>,
}

impl SRSim {
    pub fn start_sim(&self) {
        println!("Starting simulation with root module: {}", self.root_module.name);
        println!("---------------------------");
        println!("| Initializing system...  |");
        println!("---------------------------");
        self.root_module.render("".to_string());
        println!("---------------------------");
        println!("| Starting Simulation...  |");
        println!("---------------------------");
        for i in 1..10 {
            for c in &self.root_module.children {
                println!("Giving module {} some time...", c.name);
                for t in &c.threads {
                    println!("This is where I would call a thread...");
                    t();
                }
            }
        }
    }
}