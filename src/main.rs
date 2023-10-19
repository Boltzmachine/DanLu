use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use panels::Panel;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
    rc::Rc
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

mod utils;
mod run;
mod app;
mod panels;
mod logger;

use utils::KeyInputRespond;
use app::App;
use panels::PanelType;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let Some(respond) = match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        app.toggle_run_panel();
                        None
                    }
                    _ => {
                        let active_panel = app.get_active_panel();
                        Panel::<B>::handle_input(active_panel, key)
                    }
                } {
                    match respond {
                        KeyInputRespond::Activate(panel) => app.active_panel = panel,
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let tab_chunk = chunks[0];
    let content_chunk = chunks[1];

    app.draw(f, tab_chunk);
    app.chart_panel.draw(f, content_chunk, app.active_panel == PanelType::Log);

    // let selected_run = app.run_panel.runs.state.selected().unwrap();
    // app.log_panel.log = Rc::clone(&app.run_panel.runs.items[selected_run].log);
    // app.run_panel.draw(f, chunks[0], app.active_panel == PanelType::Run);
    // app.log_panel.draw(f, chunks[1], app.active_panel == PanelType::Log);
    // app.run_panel.draw(f, f.size(), app.active_panel == PanelType::Run)
}