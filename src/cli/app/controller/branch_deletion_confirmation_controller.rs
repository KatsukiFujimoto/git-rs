use crate::{
    cli::app::{
        component::branch_deletion_confirmation::BranchDeletionConfirmation,
        stateful_table::StatefulTable, Page,
    },
    domain::entity::branch::Branch,
    infrastracture::repository::GitRepository,
    usecase::branch::delete::DeleteBranch,
};
use crossterm::event::{self, Event, KeyCode};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};

pub struct BranchDeletionConfirmationController {}
impl BranchDeletionConfirmationController {
    pub fn run(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        repo: &GitRepository,
        stateful_branches: &mut StatefulTable<Branch>,
    ) -> anyhow::Result<Option<Page>> {
        loop {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(5)
                    .split(frame.size());
                BranchDeletionConfirmation::render(frame, layout[0], stateful_branches);
            })?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('y') => {
                        if let Some(branch) = stateful_branches.cursor_focused() {
                            DeleteBranch::new(repo).run(branch.clone())?;
                        }
                        break Ok(Some(Page::BranchList));
                    }
                    KeyCode::Char('n') => {
                        break Ok(Some(Page::BranchList));
                    }
                    KeyCode::Char('q') => {
                        break Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}
