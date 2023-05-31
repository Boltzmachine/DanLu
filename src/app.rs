use crate::panels::task_panel::TaskPanel;
use crate::panels::log_panel::LogPanel;
use crate::panels::Panel;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

pub struct App<'a, B: Backend> {
    pub task_panel: 
}

impl<'a, B: Backend> App<'a, B> {
    pub fn new() -> Self {
        App {
            
        }
    }
}