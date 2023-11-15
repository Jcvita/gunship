use blueteamtool::terminal::GunshipTerminal;
use ratatui::terminal::Terminal;
use ratatui::widgets::{Block, Borders};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::backend::CrosstermBackend;

use std::io::stdout;

fn main() {

    let mut gunship = GunshipTerminal::new().setup()?;



    GunshipTerminal::teardown()
}