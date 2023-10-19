use crate::panels::{Panel, PanelType, RunPanel, LogPanel, ChartPanel};
use crate::run::Run;
use ratatui::backend::Backend;
use ratatui::{
    layout::{Constraint, Direction, Rect, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal};
use sqlite::Connection;

pub struct App<'a> {
    pub active_panel: PanelType,
    pub run_panel: RunPanel,
    pub log_panel: LogPanel,
    pub chart_panel: ChartPanel,
    pub tabs: Tabs<'a>,
    pub connection: Connection,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let log_panel = LogPanel::new("");
        let run_panel = RunPanel::default();
        let chart_panel = ChartPanel::new();
        let tabs = Tabs::new(vec!["Home", "Chart"])
            .block(Block::default().title("Tabs").borders(Borders::ALL))
            .select(0)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow));

        let connection = sqlite::open("workspace.db").unwrap();

        
        let create_runs_table = "CREATE TABLE IF NOT EXISTS Runs (
                run_id INTEGER PRIMARY KEY,
                run_name TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT NOT NULL
            );";
        connection.execute(create_runs_table).unwrap();

        App {
            active_panel: PanelType::Run,
            run_panel,
            log_panel,
            chart_panel,
            tabs,
            connection,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let tabs = Tabs::new(vec!["Home", "Chart"])
            .block(Block::default().title("Tabs").borders(Borders::ALL))
            .select(1)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow));
        f.render_widget(tabs, area);
    }

    pub fn get_active_panel<B: Backend>(&mut self) -> &mut dyn Panel<B> {
        match self.active_panel {
            PanelType::Run => &mut self.run_panel,
            PanelType::Log => &mut self.log_panel,
        }
    }

    pub fn toggle_run_panel(&mut self) {
        self.run_panel.visible = !self.run_panel.visible;
        if self.run_panel.visible {
            self.run_panel.previous_active = Some(self.active_panel);
            self.active_panel = PanelType::Run;
        } else {
            self.active_panel = self.run_panel.previous_active.unwrap();
            self.run_panel.previous_active = None;
        }
    }
}