use crate::{
    cli::app::{component::branch_list::BranchList, stateful_table::StatefulTable, Page},
    infrastracture::repository::GitRepository,
    usecase::branch::get_all::GetAllBranches,
};
use crossterm::event::{self, Event, KeyCode};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};

pub struct BranchListController {}
impl BranchListController {
    pub fn run(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        repo: &GitRepository,
    ) -> anyhow::Result<Option<Page>> {
        let branches = GetAllBranches::new(repo).run()?;
        let mut stateful_branches = StatefulTable::new(branches);

        loop {
            terminal.draw(|frame| {
                let layout = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(5)
                    .split(frame.size());
                BranchList::render(frame, layout[0], &mut stateful_branches);
            })?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break Ok(None);
                    }
                    KeyCode::Char('j') | KeyCode::Down => stateful_branches.next(),
                    KeyCode::Char('k') | KeyCode::Up => stateful_branches.previous(),
                    KeyCode::Char('d') => {
                        break Ok(Some(Page::BranchDeletionConfirmation(stateful_branches)));
                    }
                    _ => {}
                }
            }
        }
    }
}
