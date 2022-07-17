use self::{
    component::{
        branch_deletion_confirmation::BranchDeletionConfirmation, branch_table::BranchTable,
    },
    stateful_table::StatefulTable,
};
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
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};

pub mod component;
pub mod stateful_table;

enum Page {
    BranchList,
    BranchDeletionConfirmation,
}

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
        let mut stateful_table = StatefulTable::new(branches_getter.run()?);
        let mut page = Page::BranchList;
        loop {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(5)
                    .split(frame.size());
                match page {
                    Page::BranchList => {
                        BranchTable::render(frame, layout[0], &mut stateful_table);
                    }
                    Page::BranchDeletionConfirmation => {
                        BranchDeletionConfirmation::render(frame, layout[0]);
                    }
                }
            })?;

            if let Event::Key(key) = event::read()? {
                match page {
                    Page::BranchList => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => stateful_table.next(),
                        KeyCode::Char('k') | KeyCode::Up => stateful_table.previous(),
                        KeyCode::Char('d') => page = Page::BranchDeletionConfirmation,
                        _ => {}
                    },
                    Page::BranchDeletionConfirmation => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('y') => {
                            if let Some(branch) = stateful_table.selected() {
                                branch_deleter.run(branch.clone())?;
                                stateful_table.set_items(branches_getter.run()?);
                                page = Page::BranchList;
                            }
                        }
                        KeyCode::Char('n') => page = Page::BranchList,
                        _ => {}
                    },
                }
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}
