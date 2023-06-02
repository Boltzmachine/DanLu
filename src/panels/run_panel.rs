use super::{Panel, PanelType};
use ratatui::backend::Backend;

use ratatui::{
    layout::{Constraint, Direction, Rect, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal};
use crossterm::event::{KeyCode, KeyEvent};
use crate::utils::KeyInputRespond;

pub struct RunPanel {
    pub visible: bool,
    pub previous_active: Option<PanelType>,
    pub input: String,
    pub input_height: u16,
}

impl Default for RunPanel {
    fn default() -> Self {
        RunPanel {
            visible: false,
            previous_active: None,
            input: String::new(),
            input_height: 1,
        }
    }
}

impl<B: Backend> Panel<B> for RunPanel {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _is_active: bool) {
        if self.visible {
            let area = self.centered_rect(60, area);
            f.render_widget(Clear, area); //this clears out the background

            let outer_block = Block::default()
                .title("Run")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green));
            let inner_area = outer_block.inner(area);
            f.render_widget(outer_block, area);

            let input = Paragraph::new(self.input.as_str())
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Command").border_style(Style::default().fg(Color::White)))
                .wrap(Wrap { trim: true });
            f.render_widget(input, inner_area);
        }
    }

    fn handle_input(&mut self, key: KeyEvent) -> Option<KeyInputRespond> {
        None
    }
}

impl RunPanel {
    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(&self, percent_x: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(self.input_height),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
