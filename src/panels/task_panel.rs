use crate::utils::task::*;
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


pub struct TaskPanel {
    pub tasks: StatefulList<Task>,
}

impl TaskPanel {
    pub fn new() -> TaskPanel {
        TaskPanel {
            tasks: StatefulList::with_items(vec![
                Task::random(),
                Task::random(),
                Task::random(),
                Task::random(),
                Task::random(),
                Task::random(),
            ]),
        }
    }
}

impl<B: Backend> Panel<B> for TaskPanel {
    fn draw(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let items: Vec<ListItem> = self
        .tasks
        .items
        .iter()
        .map(|task| {
            let status = Line::from(Span::styled(
                task.status.as_ref(),
                match task.status {
                    TaskStatus::Running => Style::default().fg(Color::Green),
                    TaskStatus::Pending => Style::default().fg(Color::Yellow),
                    TaskStatus::Finished => Style::default().fg(Color::White),
                    TaskStatus::Crashed => Style::default().fg(Color::Red),
                    TaskStatus::Killed => Style::default().fg(Color::Magenta),
                    TaskStatus::KeyboardInterrupt => Style::default().fg(Color::Cyan),
                }
            ));

            ListItem::new(vec![
                status,
            ])
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunk, &mut self.tasks.state);

    }

    fn handle_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                self.tasks.next();
            }
            KeyCode::Up => {
                self.tasks.previous();
            }
            _ => {}
        }
    }
}
