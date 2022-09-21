mod sr_std;

mod prelude {
    pub use crate::sr_std::sr_module::*;
    pub use crate::sr_std::sr_sim::*;
}

use prelude::*;
static SR_MAIN: SRModule = SRModule {
    name: "root",
    parent: None,
};
static SIM: SRSim = SRSim {root_module: &(SR_MAIN), modules: Vec::new()};

fn main() {
    sr_main();
}

fn sr_start() {
 SIM.start_sim();
}
// This a stand-in sr_main that a user would write...
fn sr_main() {
    let mut mymodule: SRModule = SRModule::new("mymodule", Some(&SR_MAIN));
    sr_start();
}
