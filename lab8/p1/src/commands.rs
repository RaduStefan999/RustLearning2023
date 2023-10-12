use std::fs;
use rusqlite::Connection;

pub trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]);
    fn init(&mut self) -> Result<(), String> { Ok(()) }
}

pub struct PingCommand {}
pub struct TimesCommand {
    pub count: u32
}
pub struct CpCommand {}

pub struct Bookmark {
    conn: rusqlite::Connection
}

impl Command for PingCommand {
    fn get_name(&self) -> &'static str { "ping" }
    fn exec(&mut self, _: &[&str]) {
        println!("pong");
    }
}

impl Command for TimesCommand {
    fn get_name(&self) -> &'static str { "times" }
    fn exec(&mut self, _: &[&str]) {
        println!("{}", self.count);
        self.count += 1;
    }
}

impl Command for CpCommand {
    fn get_name(&self) -> &'static str { "copy" }
    fn exec(&mut self, args: &[&str]) {
        if args.len() != 2 {
            println!("Copy command taxes exactly 2 arguments, {} were given", args.len());
            return;
        }
        match fs::copy(args[0], args[1]) {
            Err(err) => println!("Error on copy command {:?}", err),
            Ok(_) => ()
        }
    }
}

impl Command for Bookmark {
    fn get_name(&self) -> &'static str {
        return "bm";
    }
    fn exec(&mut self, args: &[&str]) {
        if args.len() < 2 {
            println!("Failed to run bookmark command, at least 2 arguments");
            return;
        }
        if args[0] != "add" && args[0] != "search" {
            println!("Unknown command {}", args[0]);
            return;
        }
        let cmd_res;
        if args[0] == "add" {
            cmd_res = self.exec_add(&args[1..]);
        }
        else {
            cmd_res = self.exec_search(&args[1..]);
        }

        if let Err(err) = cmd_res {
            println!("{}", err);
        }
    }
    fn init(&mut self) -> Result<(), String> {

        match Connection::open("bookmarks.db") {
            Err(err) => return Err(err.to_string()),
            Ok(val) => self.conn = val
        }
        return Ok(());
    }
}

impl Bookmark {
    fn exec_add(&mut self, args: &[&str]) -> Result<(), String> {
        Ok(())
    }

    fn exec_search(&mut self, args: &[&str]) -> Result<(), String> {
        Ok(())
    }
}