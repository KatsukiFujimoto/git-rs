use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

pub struct TableSample;
impl TableSample {
    pub fn render<B: Backend>(
        f: &mut Frame<B>,
        table_state: &mut TableState,
        branch_names: &Vec<String>,
    ) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(f.size());
        let normal_style = Style::default().bg(Color::Blue);
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let header_cells = ["Branch Name"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = branch_names.iter().map(|branch| {
            let cells = vec![Cell::from(branch.to_string())];
            Row::new(cells).height(1).bottom_margin(1)
        });
        let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        f.render_stateful_widget(table, layout[0], table_state)
    }
}
