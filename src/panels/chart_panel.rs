use super::{Panel, PanelType};
use ratatui::backend::Backend;

use ratatui::{prelude::*, widgets::*};
use ratatui::symbols;
use crossterm::event::{KeyCode, KeyEvent};
use crate::utils::KeyInputRespond;

const DATA: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];


pub struct ChartPanel {
    pub data: Vec<(f64, f64)>,
}

impl ChartPanel {
    pub fn new() -> Self {
        ChartPanel {
            data: DATA.to_vec(),
        }
    }
}

impl<B: Backend> Panel<B> for ChartPanel {
    fn draw(&mut self, f: &mut Frame<B>, area: Rect, _is_active: bool) {
        let datasets = vec![Dataset::default()
            .name("data")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(GraphType::Line)
            .data(&self.data)];
        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title("Chart 2".cyan().bold())
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 5.0])
                    .labels(vec!["0".bold(), "2.5".into(), "5.0".bold()]),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 5.0])
                    .labels(vec!["0".bold(), "2.5".into(), "5.0".bold()]),
            );
        f.render_widget(chart, area);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Option<KeyInputRespond> {
        None
    }
}
