use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use convit::app::{App, AppResult};
use convit::tui::Tui;
fn main() -> AppResult<()> {

    let mut app = App::new();

    let stderr = io::stderr();
    let backend = CrosstermBackend::new(stderr);
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);

    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
    }

    tui.exit()?;

    Ok(())
}
