use crate::prelude::*;

type SRThread = fn();

pub trait Render {
    fn render(&self, indent: String);
}

pub struct SRModule {
    pub name: String,
    pub parent: Option<Box<SRModule>>,
    pub children: Vec<Box<SRModule>>,
    pub threads: Vec<SRThread>,
}

impl SRModule {
    pub fn new(name: String, parent: Option<Box<SRModule>>) -> Self {
        Self {
            name,
            parent,
            children: vec![],
            threads: vec![],
        }
    }
}

impl Render for SRModule {
    fn render(&self, indent: String) {
        let indent_local = indent + "  ";
        println!("{}Module: {}", indent_local.as_str(), &self.name);
        if !&self.children.is_empty() {
            for c in &self.children {
                println!("{}child: {}", indent_local.as_str(), c.name);
                c.render(indent_local.as_str().to_string());
            }
        } else {
            println!("{}no children", indent_local.as_str());
        }
    }
}
