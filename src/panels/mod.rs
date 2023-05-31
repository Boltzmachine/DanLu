pub mod task_panel;
pub mod log_panel;

use crate::utils::task::Task;
use crate::utils::stateful_list::StatefulList;

use task_panel::TaskPanel;

use ratatui::{Frame, backend::Backend, layout::Rect};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};


pub trait Panel<B: Backend> {
    fn draw(&mut self, f: &mut Frame<B>, chunk: Rect);
    fn handle_input(&mut self, key: KeyEvent);
}