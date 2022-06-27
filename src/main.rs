use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Row, Table},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let table = Table::new(vec![
            Row::new(vec!["row1_1", "row1_2", "row1_3"]),
            Row::new(vec!["row2_1", "row2_2", "row2_3"]),
            Row::new(vec!["row3_1", "row3_2", "row3_3"]),
            Row::new(vec!["row4_1", "row4_2", "row4_3"]),
            Row::new(vec!["row5_1", "row5_2", "row5_3"]),
            Row::new(vec!["row6_1", "row6_2", "row6_3"]),
            Row::new(vec!["row7_1", "row7_2", "row7_3"]),
            Row::new(vec!["row8_1", "row8_2", "row8_3"]),
            Row::new(vec!["row9_1", "row9_2", "row9_3"]),
        ])
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["column1", "column2", "column3"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(Block::default().title("Table"))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(20),
        ])
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");
        f.render_widget(table, size);
    })?;

    thread::sleep(Duration::from_millis(10000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
