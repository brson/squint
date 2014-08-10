pub use parser::parse;

pub mod ast {
    #[deriving(Show)]
    pub struct SquintaxTree {
        pub agents: Vec<Agent>
    }

    #[deriving(Show)]
    pub struct Agent {
        pub name: String,
        pub init_script: Script,
        pub scripts: Vec<Script>,
    }

    #[deriving(Show)]
    pub struct Script {
        pub name: String,
        pub instrs: Vec<Instr>
    }

    #[deriving(Show)]
    pub enum Instr {
        At(Moment, String)
    }

    #[deriving(Show)]
    pub struct Moment {
        pub hours: u8,
        pub minutes: u8,
        pub seconds: u8,
        pub relative: bool
    }
}

mod parser {
    use ast;
    use lexer;
    use std::io;

    pub fn parse<B: io::Buffer>(rdr: &mut B) -> ast::SquintaxTree {
        for line in rdr.lines() {
            let line = line.ok().unwrap();
            let tokens = lexer::tokenize_line(line.as_slice());
            println!("{}", tokens);
        }

        ast::SquintaxTree { agents: vec!() }
    }
}

mod lexer {
    #[deriving(Show)]
    pub enum Token {
        Agent,
        At,
        Confusion(String),
        Moment(u8, u8, u8, bool),
        Name(String),
        Silence,
        QuotedString(String),
        UnquotedString(String)
    }

    pub fn tokenize_line(line: &str) -> Vec<Token> {
        let mut buf = vec!();
        tokenize_line_buf(&mut buf, line);
        return buf;
    }

    fn tokenize_line_buf(buf: &mut Vec<Token>, line: &str) {
        let line = line.trim();

        if line.is_empty() {
            buf.push(Silence);
        } else if line.starts_with("agent") {
            buf.push(Agent);
            let line = line.slice_from("agent".len()).trim();
            read_name(buf, line)
        } else if line.starts_with("at") {
            buf.push(At);
            let line = line.slice_from("at".len()).trim();
            tokenize_at_command(buf, line);
        } else {
            buf.push(Confusion(line.to_string()))
        }
    }

    fn tokenize_at_command(buf: &mut Vec<Token>, line: &str) {
        let line = tokenize_moment(buf, line);
        buf.push(UnquotedString(line.to_string()));
    }

    fn tokenize_moment<'a>(buf: &mut Vec<Token>, line: &'a str) -> &'a str {
        for word in line.splitn(' ', 1) {
            let parts = word.split(':').collect::<Vec<&str>>();
            if parts.len() != 3 {
                buf.push(Confusion(line.to_string()));
                return line.slice(0, 0);
            }
            if parts[0].is_empty() {
                buf.push(Confusion(line.to_string()));
                return line.slice(0, 0);
            }
            let mut hours_str = parts[0];
            let minutes_str = parts[1];
            let seconds_str = parts[2];
            let mut relative = false;
            if parts[0].char_at(0) == '+' {
                relative = true;
                hours_str = hours_str.slice(1, hours_str.len());
            }
            let maybe_hours: Option<u8> = from_str(hours_str);
            let maybe_minutes: Option<u8> = from_str(minutes_str);
            let maybe_seconds: Option<u8> = from_str(seconds_str);

            match (maybe_hours, maybe_minutes, maybe_seconds) {
                (Some(hours), Some(minutes), Some(seconds)) => {
                    buf.push(Moment(hours, minutes, seconds, relative));
                    return line.slice(word.len(), line.len()).trim();
                }
                _ => ()
            }
        }

        buf.push(Confusion(line.to_string()));
        return line.slice(0, 0);
    }

    fn read_name(buf: &mut Vec<Token>, line: &str) {
        for char in line.chars() {
            if char.is_whitespace() {
                buf.push(Confusion(line.to_string()))
            }
        }

        buf.push(Name(line.to_string()))
    }
}
