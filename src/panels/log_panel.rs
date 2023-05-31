use super::Panel;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crossterm::event::{KeyCode, KeyEvent};

pub struct LogPanel {
    pub log: &str,
}

impl<B: Backend> Panel<B> for LogPanel {
    fn draw(&mut self, f: &mut Frame<B>, chunk: Rect) {
        if let Some(selected_task) = app.panels[0].tasks.state.selected() {
            let log = Paragraph::new(Line::from(app.panels[0].tasks.items[selected_task].log.as_ref()))
                .block(Block::default().borders(Borders::ALL).title("Log"))
                .wrap(Wrap { trim: true });
            f.render_widget(log, chunk);
        }
    }

    fn handle_input(&mut self, key: KeyEvent) {
    }
}