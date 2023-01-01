mod sr_std;

mod prelude {
    pub use crate::sr_std::sr_module::*;
    pub use crate::sr_std::sr_sim::*;
}

use prelude::*;

fn main() {
    let mut root_module = &Box::new(SRModule::new("rs_main".to_string(), None));
    let mut srsim = Box::new(SRSim {root_module });
    sr_main(srsim);
}

// This a stand-in sr_main that a user would write...
fn core_srt() {
    println!("hello from core!");
}
fn mem_srt() {
    println!("hello from mem!");
}

fn sr_main(mut sim: Box<SRSim>) {
    // let mut root_module = sim.root_module;
    let mut root_module = Box::new(SRModule::new("root".to_string(), None));
    let mut core_module = Box::new(SRModule::new("core".to_string(), None));
    let mut mem_module = Box::new(SRModule::new("mem".to_string(), None));

    let mut core_srthread: fn() = core_srt;
    let mut mem_srthread: fn() = mem_srt;
    let mut rm = root_module;

    core_module.threads.push(core_srthread);
    core_module.threads.push(mem_srthread);

    // root_module.children.push(core_module);
    // root_module.children.push(mem_module);

    root_module.children.push(core_module);
    root_module.children.push(mem_module);

    sim.start_sim();
}
