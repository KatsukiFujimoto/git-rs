use self::{
    controller::branch_deletion_confirmation_controller::BranchDeletionConfirmationController,
    stateful_table::StatefulTable,
};
use crate::{
    cli::app::controller::branch_list_controller::BranchListController,
    domain::entity::branch::Branch, infrastracture::repository::GitRepository,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::Repository;
use std::{io, path::Path};
use tui::{backend::CrosstermBackend, Terminal};

pub mod component;
pub mod controller;
pub mod stateful_table;

pub enum Page {
    BranchList,
    BranchDeletionConfirmation(StatefulTable<Branch>),
}

pub struct App {
    pub page: Option<Page>,
}
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
impl App {
    pub fn new() -> Self {
        Self {
            page: Some(Page::BranchList),
        }
    }

    pub fn start(&mut self, git_path: &dyn AsRef<Path>) -> anyhow::Result<()> {
        // set up terminal
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;

        // run application
        let git_repo = Repository::open(git_path)?;
        let repo = GitRepository::new(&git_repo);
        while let Some(page) = &mut self.page {
            let next_page = match page {
                Page::BranchList => BranchListController::run(&mut terminal, &repo)?,
                Page::BranchDeletionConfirmation(stateful_branches) => {
                    BranchDeletionConfirmationController::run(
                        &mut terminal,
                        &repo,
                        stateful_branches,
                    )?
                }
            };
            self.page = next_page;
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}
