mod commands;
mod terminal;

fn main() {
    let mut app_terminal = terminal::Terminal::new();
    app_terminal.register(Box::new(commands::PingCommand{}));
    app_terminal.register(Box::new(commands::TimesCommand{count: 0}));
    app_terminal.register(Box::new(commands::CpCommand{}));
    app_terminal.run();

}
