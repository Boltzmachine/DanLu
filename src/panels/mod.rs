pub mod task_panel;
pub mod log_panel;
pub mod run_panel;

pub use task_panel::TaskPanel;
pub use log_panel::LogPanel;
pub use run_panel::RunPanel;

use crate::utils::KeyInputRespond;

use ratatui::{Frame, backend::Backend, layout::Rect};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};

#[derive(PartialEq, Clone, Copy)]
pub enum PanelType {
    Task,
    Log,
    Run,
}

pub trait Panel<B: Backend> {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, is_active: bool);
    fn handle_input(&mut self, key: KeyEvent) -> Option<KeyInputRespond>;
}