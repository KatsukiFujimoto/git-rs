use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct BranchDeletionConfirmation;
impl BranchDeletionConfirmation {
    pub fn render<B: Backend>(frame: &mut Frame<B>, area: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(area);
        let text = vec![];
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Branch Deletion Confirmation")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(tui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, layout[0]);
    }
}
