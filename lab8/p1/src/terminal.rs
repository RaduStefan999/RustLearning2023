use std::io;
use crate::commands::Command;

pub struct Terminal {
    suported_commands: Vec<Box<dyn Command>>
}

impl Terminal {
    pub fn new() -> Terminal {
        return Terminal{suported_commands: vec![]}
    }

    pub fn register(&mut self, cmd: Box<dyn Command>) {
        self.suported_commands.push(cmd);
    }

    fn process_cmd(&mut self, cmd: &str) -> bool {
        let args: Vec<&str> = cmd.split(" ").collect();
        if args.len() == 0 {
            println!("Invalid command");
            return true;
        }

        if args[0] == "stop" {
            return false;
        }

        for command_obj in self.suported_commands.iter_mut() {
            if args[0] == command_obj.get_name() {
                command_obj.exec(&args[1..]);
                return true;
            }
        }

        println!("Unknown command {}", args[0]);
        return true;
    }

    pub fn run(&mut self) {
        let mut cmd_buf = String::new();

        loop {
            cmd_buf.clear();
            if let Err(err) = io::stdin().read_line(&mut cmd_buf) {
                println!("{}", err);
                return;
            }
            if !self.process_cmd(cmd_buf.trim()) {
                println!("Terminal was stopped");
                return;
            }
        }
    }
}