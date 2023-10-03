trait Command {
    fn get_name() -> &str;
    fn exec(args: &[&str]);
}

struct PingCommand {}
struct TimesCommand {
    count: u32
}
struct CpCommand {}

impl Command for PingCommand {
    fn get_name() -> &str { "ping" }
    fn exec(args: &[&str]) {
        println!("pong");
    }
}

impl TimesCommand {
    
}