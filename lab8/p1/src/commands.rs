use std::fs;
use rusqlite::Connection;

pub trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]);
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
    fn get_name(&self) -> &'static str { "cp" }
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
}

impl Bookmark {
    pub fn new(db_name: &str) -> Result<Bookmark, String> {
        let bookmark;

        match Connection::open(db_name) {
            Err(err) => return Err(err.to_string()),
            Ok(val) => bookmark = Bookmark{ conn: val }
        }

        match bookmark.init_db() {
            Err(err) => return Err(err),
            Ok(_) => return Ok(bookmark)
        }
    }

    fn init_db(&self) -> Result<(), String> {
        let create_stmt = r"
        CREATE TABLE IF NOT EXISTS bookmarks (
            name text not null,
            link text not null
        );
        ";
        if let Err(err) = self.conn.execute(create_stmt, ()) {
            return Err(err.to_string());
        }
        return Ok(());
    }

    fn exec_add(&mut self, args: &[&str]) -> Result<(), String> {
        if args.len() != 2 {
            println!("add accepts exactly 2 arguments");
        }
        let insert_smt = r"
        INSERT INTO bookmarks (name, link) VALUES (?1, ?2)
        ";
        if let Err(err) = self.conn.execute(insert_smt, (args[0], args[1])) {
            return Err(err.to_string());
        }
        Ok(())
    }

    fn exec_search(&mut self, args: &[&str]) -> Result<(), String> {
        if args.len() != 1 {
            println!("search accepts exactly 1 argument");
        }

        let select_smt = self.conn.prepare(r"
        SELECT link FROM bookmarks WHERE name=?1
        ");
        if let Err(err) = select_smt {
            return Err(err.to_string());
        }
        let mut select_smt = select_smt.unwrap();

        let bookmark_iter = select_smt.query_map((args[0], ), |row| {
            let link: String = row.get(0)?;
            Ok(link)
        });
        if let Err(err) = bookmark_iter {
            return Err(err.to_string());
        }
        let bookmark_iter = bookmark_iter.unwrap();

        for link in bookmark_iter {
            if let Ok(link_val) = link {
                println!("Found bookmark: {}", link_val)
            }
        }

        Ok(())
    }
}