mod commands;
mod terminal;

fn main() {
    let bookmark_obj = commands::Bookmark::new("bookmarks.db");
    if let Err(err) = bookmark_obj {
        println!("Failed to create bookmarks.db {:?}", err);
        return;
    }

    let mut app_terminal = terminal::Terminal::new();
    app_terminal.register(Box::new(commands::PingCommand{}));
    app_terminal.register(Box::new(commands::TimesCommand{count: 0}));
    app_terminal.register(Box::new(commands::CpCommand{}));
    app_terminal.register(Box::new(bookmark_obj.unwrap()));
    app_terminal.run();

}
