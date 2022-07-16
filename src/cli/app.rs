use self::{stateful_table::StatefulTable, view::branch_table::BranchTable};
use crate::{
    infrastracture::repository::GitRepository,
    usecase::branch::{delete::DeleteBranch, get_all::GetAllBranches},
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::Repository;
use std::{io, path::Path};
use tui::{backend::CrosstermBackend, Terminal};

pub mod stateful_table;
pub mod view;

pub struct App {}
impl App {
    pub fn start(git_path: &dyn AsRef<Path>) -> anyhow::Result<()> {
        // set up terminal
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;

        // run application
        let git_repo = Repository::open(git_path)?;
        let repo = GitRepository::new(&git_repo);
        let branches_getter = GetAllBranches::new(&repo);
        let branch_deleter = DeleteBranch::new(&repo);
        let branches = branches_getter.run()?;
        let mut stateful_table = StatefulTable::new(branches);
        loop {
            terminal.draw(|frame| {
                BranchTable::render(frame, &mut stateful_table);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => stateful_table.next(),
                    KeyCode::Char('k') | KeyCode::Up => stateful_table.previous(),
                    KeyCode::Char('d') => {
                        if let Some(branch) = stateful_table.selected() {
                            branch_deleter.run(branch.clone())?;
                            stateful_table.set_items(branches_getter.run()?);
                        }
                    }
                    _ => {}
                }
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}
