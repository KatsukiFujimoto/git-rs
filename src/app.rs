use crate::table_sample::TableSample;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Branch, BranchType, Repository};
use std::{env, io};
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};

pub struct App<'a> {
    pub state: TableState,
    pub repository: &'a Repository,
    pub branches: Vec<(Branch<'a>, BranchType)>,
}
impl<'a> App<'a> {
    pub fn new(repository: &'a Repository) -> App<'a> {
        let branches = repository
            .branches(Some(BranchType::Local))
            .unwrap()
            .map(|x| x.unwrap())
            .collect();
        App {
            state: TableState::default(),
            repository,
            branches,
        }
    }

    pub fn refresh_branches(&mut self) {
        self.branches = self
            .repository
            .branches(Some(BranchType::Local))
            .unwrap()
            .map(|x| x.unwrap())
            .collect();
    }

    pub fn selected_branch(&mut self) -> Option<&mut Branch<'a>> {
        match self.state.selected() {
            Some(i) => Some(&mut self.branches[i].0),
            _ => None,
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
        let path = env::current_dir().unwrap();
        let repository = Repository::open(path).unwrap();
        let mut app = App::new(&repository);
        loop {
            terminal.draw(|f| {
                TableSample::render(
                    f,
                    &mut app.state,
                    &app.branches
                        .iter()
                        .map(|x| x.0.name().unwrap().unwrap().to_string())
                        .collect(),
                );
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('d') => {
                        if let Some(branch) = app.selected_branch() {
                            branch.delete().unwrap();
                            app.refresh_branches();
                        }
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
                if i >= self.branches.len() - 1 {
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
                    self.branches.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
