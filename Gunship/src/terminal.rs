use std::{io::{self, Stdout, stdout}, sync::{Arc, Mutex}, time::Duration, ops::{Deref, DerefMut}};
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }, event::{self, Event}, ExecutableCommand
};
use ratatui::{Terminal, prelude::{CrosstermBackend, Layout, Direction, Constraint}, widgets::{Block, Borders}, Frame, CompletedFrame};

use crate::AppState;

pub struct GunshipTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    quit_signal: bool
}

impl GunshipTerminal {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            quit_signal: false
        }
    }
    
    pub fn setup(&mut self) -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen);
        self.terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(self.terminal)
    }

    pub fn _draw_once<F>(&mut self, f: F) -> io::Result<CompletedFrame>
        where F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)
    }

    pub fn start(&mut self, ui_root: F) 
        where F: FnOnce(&mut Frame)    
    {
        while !self.quit_signal {
            self._draw_once(ui_root);
        }
    }
    
    pub fn teardown() -> Result<(), io::Error> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn next_event(timeout: Duration) -> io::Result<Option<Event>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }
        let event = event::read()?;
        Ok(Some(event))
    }
}

impl Deref for GunshipTerminal {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for GunshipTerminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for GunshipTerminal {
    fn drop(&mut self) {
        let _ = GunshipTerminal::teardown();
    }
}

pub fn run(frame: &mut Frame) -> Result<(), io::Error> {
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

        f.render_widget(left_block, layout[0]);
        f.render_widget(right_block, layout[1]);
    }

}