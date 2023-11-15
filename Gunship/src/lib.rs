use std::{io, sync::{Arc, Mutex}};
use terminal::*;

use winit::event_loop::EventLoop;
pub mod graphics;
pub mod terminal;

pub struct User<'a> {
    pub name: String,
    pub files: Vec<&'a File<'a>>,
}

pub struct File<'a> {
    pub path: String,
    pub creator: &'a User<'a>,
}

pub struct AppState<'a> {
    pub files: Vec<&'a File<'a>>,
    pub users: Vec<&'a User<'a>>,
}

pub async fn run_graphics() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::Window::new(&event_loop).unwrap();
    
    //graphics::run(event_loop, window).await
}

pub fn run_tui(state: Arc<Mutex<AppState>>) -> Result<(), io::Error> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal, state)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}