use std::{io::{self, Stdout}, sync::{Arc, Mutex}};
use crossterm::{
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use ratatui::{Terminal, prelude::{CrosstermBackend, Layout, Direction, Constraint}, widgets::{Block, Borders}};

use crate::AppState;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    state: Arc<Mutex<AppState>>
) -> Result<(), io::Error> {
    loop {
        let left_block = Block::default()
            .title("Left Section")
            .borders(Borders::ALL);

        let right_block = Block::default()
            .title("Right Section")
            .borders(Borders::ALL);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref())
            .split(terminal.size().unwrap());

        // terminal.clear().unwrap();

        terminal.clear().unwrap();

        terminal.draw(|mut f| {
            f.render_widget(left_block, layout[0]);
            f.render_widget(right_block, layout[1]);
        }).unwrap();
    }

}