use crate::table_sample::TableSample;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Branch, BranchType, Repository};
use std::{env, io, path::PathBuf};
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};

pub struct App {
    pub state: TableState,
    pub repository: Repository,
    pub branch_names: Vec<String>,
}
impl App {
    pub fn new(path: PathBuf) -> App {
        let repository = Repository::open(path).unwrap();
        let branch_names = repository
            .branches(Some(BranchType::Local))
            .unwrap()
            .map(|x| x.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();
        App {
            state: TableState::default(),
            repository,
            branch_names,
        }
    }

    pub fn refresh_branch_names(&mut self) {
        self.branch_names = self
            .repository
            .branches(Some(BranchType::Local))
            .unwrap()
            .map(|x| x.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();
    }

    pub fn selected_branch_name(&self) -> Option<&String> {
        self.state.selected().and_then(|i| self.branch_names.get(i))
    }

    pub fn start() -> Result<(), io::Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let path = env::current_dir().unwrap();
        let mut app = App::new(path);
        loop {
            terminal.draw(|f| {
                TableSample::render(f, &mut app.state, &app.branch_names);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('d') => {
                        if let Some(branch_name) = app.selected_branch_name() {
                            let mut branch: Branch = app
                                .repository
                                .find_branch(branch_name, BranchType::Local)
                                .unwrap();
                            branch.delete().unwrap();
                        }
                        app.refresh_branch_names();
                    }
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
                if i >= self.branch_names.len() - 1 {
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
                    self.branch_names.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
