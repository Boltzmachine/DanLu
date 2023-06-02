use crate::panels::{Panel, PanelType, TaskPanel, LogPanel, RunPanel, run_panel};
use ratatui::backend::Backend;

pub struct App {
    pub active_panel: PanelType,
    pub task_panel: TaskPanel,
    pub log_panel: LogPanel,
    pub run_panel: RunPanel,
}

impl App {
    pub fn new() -> Self {
        let task_panel = TaskPanel::new();
        let log_panel = LogPanel::new("");
        let run_panel = RunPanel::default();
        App {
            active_panel: PanelType::Task,
            task_panel,
            log_panel,
            run_panel,
        }
    }

    pub fn get_active_panel<B: Backend>(&mut self) -> &mut dyn Panel<B> {
        match self.active_panel {
            PanelType::Task => &mut self.task_panel,
            PanelType::Log => &mut self.log_panel,
            PanelType::Run => &mut self.run_panel,
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