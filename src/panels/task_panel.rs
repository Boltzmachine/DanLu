use crate::utils::KeyInputRespond;
use crate::jobs::run::{Run, RunStatus};
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

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        state.select(Some(0));
        StatefulList {
            state,
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}


pub struct RunPanel {
    pub runs: StatefulList<Run>,
}

impl RunPanel {
    pub fn new() -> RunPanel {
        RunPanel {
            runs: StatefulList::with_items(vec![
                Run::random(),
                Run::random(),
                Run::random(),
                Run::random(),
                Run::random(),
                Run::random(),
            ]),
        }
    }
}

impl<B: Backend> Panel<B> for RunPanel {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, is_active: bool) {
        let items: Vec<ListItem> = self
        .runs
        .items
        .iter()
        .map(|run| {
            let status = Line::from(Span::styled(
                run.status.as_ref(),
                match run.status {
                    RunStatus::Running => Style::default().fg(Color::Green),
                    RunStatus::Pending => Style::default().fg(Color::Yellow),
                    RunStatus::Finished => Style::default().fg(Color::White),
                    RunStatus::Crashed => Style::default().fg(Color::Red),
                    RunStatus::Killed => Style::default().fg(Color::Magenta),
                    RunStatus::KeyboardInterrupt => Style::default().fg(Color::Cyan),
                }
            ));

            ListItem::new(vec![
                status,
            ])
        })
        .collect();
    
    let items = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Runs")
            .border_style(Style::default().fg(if is_active { Color::Green } else { Color::White })))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, area, &mut self.runs.state);

    }

    fn handle_input(&mut self, key: KeyEvent) -> Option<KeyInputRespond> {
        match key.code {
            KeyCode::Down => {
                self.runs.next();
            }
            KeyCode::Up => {
                self.runs.previous();
            }
            KeyCode::Enter | KeyCode::Right => {
                return Some(KeyInputRespond::Activate(super::PanelType::Log));
            }
            _ => {}
        }
        None
    }
}
