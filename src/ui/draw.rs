use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::{
    algorithms::algorithm::{Algorithm, AlgorithmWrapper},
    app::App,
};

use super::{footer, header};

/// Draws to the main frame.
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

    header::draw(f, chunks[1]);

    app.get_current_tab().algorithm.draw(f, chunks[2], app);

    footer::draw(f, chunks[3], app, app.get_current_tab());
}
