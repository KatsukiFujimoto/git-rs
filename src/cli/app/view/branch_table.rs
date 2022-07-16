use crate::cli::app::stateful_table::StatefulTable;
use crate::domain::entity::branch::Branch;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub struct BranchTable;
impl BranchTable {
    pub fn render<B: Backend>(frame: &mut Frame<B>, stateful_table: &mut StatefulTable<Branch>) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(frame.size());
        let header =
            Row::new([Cell::from("Branch Name")]).style(Style::default().bg(Color::DarkGray));
        let rows = stateful_table
            .items
            .iter()
            .map(|branch| Row::new(vec![Cell::from(branch.name.as_ref())]));
        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Branch Table"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .widths(&[Constraint::Percentage(100)]);
        frame.render_stateful_widget(table, layout[0], &mut stateful_table.state);
    }
}
