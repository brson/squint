pub use parser::parse;

pub mod ast {
    use lexer;

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
        At(Moment, String),
        Confusion(Vec<lexer::Token>)
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

    enum State {
        LookingForAgent,
        ReadingInitScript(String, Vec<ast::Instr>),
        ReadingScript(String, ast::Script, String, Vec<ast::Instr>)
    }

    pub fn parse<B: io::Buffer>(rdr: &mut B) -> ast::SquintaxTree {

        let mut state = LookingForAgent;
        let mut agents = vec!();

        for line in rdr.lines() {
            let line = line.ok().unwrap();
            let tokens = lexer::tokenize_line(line.as_slice());
            println!("{}", tokens);
            let (n, agent) = next_state(state, tokens);
            state = n;
            if agent.is_some() { agents.push(agent.unwrap()) }
        }

        ast::SquintaxTree { agents: vec!() }
    }

    fn next_state(state: State, tokens: Vec<lexer::Token>) -> (State, Option<ast::Agent>) {
        match state {
            LookingForAgent => match tokens.as_slice() {
                [lexer::Agent, lexer::Name(ref s)] => {
                    (ReadingInitScript(s.to_string(), vec!()), None)
                }
                _ => (LookingForAgent, None)
            },
            ReadingInitScript(name, mut instrs) => match tokens.as_slice() {
                [lexer::Agent, lexer::Name(ref s)] => {
                    let init_script = ast::Script {
                        name: "init".to_string(),
                        instrs: instrs
                    };
                    let agent = ast::Agent {
                        name: name,
                        init_script: init_script,
                        scripts: vec!()
                    };
                    (ReadingInitScript(s.to_string(), vec!()), Some(agent))
                }
                [lexer::At, lexer::Moment(..), lexer::UnquotedString(_)] => {
                    fail!()
                }
                [lexer::Script, lexer::QuotedString(ref name)] => {
                    fail!()
                }
                tokens => {
                    instrs.push(ast::Confusion(tokens.to_vec()));
                    (ReadingInitScript(name, instrs), None)
                }
            },
            _ => fail!()
        }
    }
}

mod lexer {
    #[deriving(Show, Clone)]
    pub enum Token {
        Agent,
        At,
        Confusion(String),
        Moment(u8, u8, u8, bool),
        Name(String),
        Script,
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
        } else if line.starts_with("script") {
            buf.push(Script);
            read_quoted_string(buf, line)
        }else {
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

    fn read_quoted_string(buf: &mut Vec<Token>, line: &str) {
    }
}
