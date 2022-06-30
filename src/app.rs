use crate::table_sample::TableSample;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};

pub struct App<'a> {
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>,
}
impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![
                vec!["Row1_1", "Row1_2", "Row1_3"],
                vec!["Row2_1", "Row2_2", "Row2_3"],
                vec!["Row3_1", "Row3_2", "Row3_3"],
                vec!["Row4_1", "Row4_2", "Row4_3"],
                vec!["Row5_1", "Row5_2", "Row5_3"],
                vec!["Row6_1", "Row6_2", "Row6_3"],
                vec!["Row7_1", "Row7_2", "Row7_3"],
                vec!["Row8_1", "Row8_2", "Row8_3"],
                vec!["Row9_1", "Row9_2", "Row9_3"],
            ],
        }
    }

    pub fn start() -> Result<(), io::Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let mut app = App::new();
        loop {
            terminal.draw(|f| {
                TableSample::render(f, &mut app);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                }
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
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
