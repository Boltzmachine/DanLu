use crate::utils::KeyInputRespond;

use super::Panel;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent};
use crate::panels::PanelType;

pub struct LogPanel {
    scroll: u16,
    pub log: Rc<String>,
}

impl LogPanel {
    pub fn new(log: &'static str) -> LogPanel {
        LogPanel {
            scroll: 0,
            log: Rc::new(log.to_string()),
        }
    }
}

impl<B: Backend> Panel<B> for LogPanel {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, is_active: bool) {
        let log = Paragraph::new(Line::from((*self.log).as_str()))
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Log")
                .border_style(Style::default().fg(if is_active { Color::Green } else { Color::White })))
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0));
        f.render_widget(log, area);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Option<KeyInputRespond> {
        match key.code {
            KeyCode::Left => { return Some(KeyInputRespond::Activate(PanelType::Run)); }
            KeyCode::Down => { self.scroll += 1; }
            KeyCode::Up => { 
                if self.scroll > 0 {
                    self.scroll -= 1;
                }
            }
            _ => {}
        }
        None
    }
}