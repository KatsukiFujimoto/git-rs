use crate::cli::app::stateful_table::StatefulTable;
use crate::domain::entity::branch::Branch;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub struct BranchTable;
impl BranchTable {
    pub fn render<B: Backend>(
        frame: &mut Frame<B>,
        area: Rect,
        stateful_table: &mut StatefulTable<Branch>,
    ) {
        let header = Row::new([
            Cell::from("Branch Name"),
            Cell::from("Branch Type"),
            Cell::from("Current Branch"),
        ])
        .style(Style::default().bg(Color::DarkGray));
        let rows = stateful_table.items.iter().map(|branch| {
            Row::new(vec![
                Cell::from(branch.name.as_ref()),
                Cell::from(branch.branch_type()),
                Cell::from(if branch.current { "true" } else { "" }),
            ])
        });
        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Branch Table"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .widths(&[
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ]);
        frame.render_stateful_widget(table, area, &mut stateful_table.state);
    }
}
