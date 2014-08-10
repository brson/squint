use std::io;

#[deriving(Show)]
pub struct SquintaxTree {
    agents: Vec<Agent>
}

#[deriving(Show)]
pub struct Agent {
    name: String,
    init_script: Script,
    scripts: Vec<Script>,
}

#[deriving(Show)]
pub struct Script {
    name: String,
    instrs: Vec<Instr>
}

#[deriving(Show)]
pub enum Instr {
    At(Moment, String)
}

#[deriving(Show)]
pub struct Moment {
    hours: u8,
    minutes: u8,
    seconds: u8,
    relative: bool
}

impl SquintaxTree {
    pub fn parse<B: io::Buffer>(rdr: &mut B) -> SquintaxTree {
        for line in rdr.lines() {
            let line = line.ok().unwrap();
            let line = line.as_slice().trim();
            println!("{}", line);
        }

        SquintaxTree { agents: vec!() }
    }
}
