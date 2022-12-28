use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Gauge, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::{
    algorithms::algorithm::{Algorithm, AlgorithmWrapper},
    app::{App, InputMode, Tab},
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let titles = app
        .tab_state
        .tabs
        .iter()
        .map(|tab| {
            let title = match &tab.algorithm {
                AlgorithmWrapper::GrahamScan(algorithm) => Spans::from(Span::styled(
                    algorithm.get_title(),
                    Style::default().fg(Color::Gray),
                )),
            };
            title
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tab_state.index);
    f.render_widget(tabs, chunks[0]);

    draw_header(f, chunks[1]);

    app.get_current_tab().algorithm.draw(f, chunks[2], app);

    draw_footer(f, chunks[3], app, app.get_current_tab());
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

fn draw_progress<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let mut ratio = 0.0;
    if app.get_current_tab().algorithm.get_maximum_step_count() > 0 {
        ratio = app.get_current_tab().step as f64
            / (app.get_current_tab().algorithm.get_maximum_step_count() - 1) as f64;
    }
    let label = format!("{:.2}%", ratio * 100.0);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .label(label)
        .ratio(ratio);
    f.render_widget(gauge, area);
}

fn draw_footer<B>(f: &mut Frame<B>, area: Rect, app: &App, _tab: &Tab)
where
    B: Backend,
{
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(3)].as_ref())
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(vertical_chunks[1]);

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
    f.render_widget(help_message, vertical_chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(block);
    f.render_widget(input, horizontal_chunks[0]);

    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => f.set_cursor(
            horizontal_chunks[0].x + app.input.len() as u16 + 1,
            horizontal_chunks[0].y + 1,
        ),
    }

    draw_progress(f, horizontal_chunks[1], app);
}
