use crate::prelude::*;
use crate::{SIM, SR_MAIN};

//TODO Create static SRodule sr_main root module
pub struct SRModule<'a> {
    pub name: &'a str,
    pub parent: Option<&'a SRModule<'a>>,
}

impl<'a> SRModule<'a> {
    pub fn new(name: &'a str, parent: Option<&'a SRModule>) -> Self {
        let s = Self {
            name,
            parent,
        };
        //SIM.modules.append(&s);
        s
    }
}

