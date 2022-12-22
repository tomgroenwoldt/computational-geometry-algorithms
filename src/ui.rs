use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    algorithms::algorithm::Algorithm,
    app::{App, InputMode},
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_header(f, chunks[0]);
    app.algorithm.draw_algorithm(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![Spans::from(
        "This will be the section explaining the graham scan algorithm.",
    )];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Description", Style::default()));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_footer<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(3)].as_ref())
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Point amount", Style::default()));

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to edit the point amount."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start the algorithm."),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(block);
    f.render_widget(input, chunks[1]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => {
            f.set_cursor(chunks[1].x + app.input.len() as u16 + 1, chunks[1].y + 1)
        }
    }
}
