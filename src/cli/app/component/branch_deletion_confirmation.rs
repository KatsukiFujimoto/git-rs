use super::selected_branch_list::SelectedBranchList;
use crate::{cli::app::stateful_table::StatefulTable, domain::entity::branch::Branch};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct BranchDeletionConfirmation;
impl BranchDeletionConfirmation {
    pub fn render<B: Backend>(
        frame: &mut Frame<B>,
        area: Rect,
        stateful_branches: &mut StatefulTable<Branch>,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .margin(5)
            .split(area);
        let text = vec![
            Spans::from(vec![Span::raw(
                "Are you sure to delete the following branch?",
            )]),
            Spans::from(vec![Span::raw("Type 'y' to continue.")]),
            Spans::from(vec![Span::raw("Type 'n' to cancell.")]),
        ];
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Branch Deletion Confirmation")
                    .borders(Borders::ALL),
            )
            .style(Style::default().bg(Color::Black))
            .alignment(tui::layout::Alignment::Left);
        frame.render_widget(paragraph, layout[0]);
        SelectedBranchList::render(frame, layout[1], stateful_branches.selected())
    }
}
