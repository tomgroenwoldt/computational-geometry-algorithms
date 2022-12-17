use itertools::Itertools;
use nalgebra::Point2;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Points, Rectangle},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::algorithms::graham_scan::{scan_lower, scan_upper};

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_text(f, chunks[0]);
    draw_line(f, chunks[1]);
    draw_text(f, chunks[2]);
}

fn draw_line<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let map = Canvas::default()
        .block(Block::default().title("Algorithm").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.layer();
            let mut points = vec![
                Point2::new(-180.0, -90.0),
                Point2::new(-50.0, -20.0),
                Point2::new(55.0, -70.0),
                Point2::new(-15.0, 20.0),
                Point2::new(-105.0, 50.0),
                Point2::new(70.0, 30.0),
                Point2::new(-50.0, 20.0),
                Point2::new(55.0, 70.0),
                Point2::new(-35.0, -20.0),
                Point2::new(105.0, -50.0),
            ];

            for point in &points {
                ctx.print(
                    point.x,
                    point.y,
                    Span::styled("⚫", Style::default().fg(Color::White)),
                );
            }

            let upper_points = scan_upper(&mut points.clone());
            let lower_points = scan_lower(&mut points);

            upper_points
                .into_iter()
                .tuple_windows()
                .for_each(|(from, to)| {
                    ctx.draw(&Line {
                        x1: from.x,
                        x2: to.x,
                        y1: from.y,
                        y2: to.y,
                        color: Color::Blue,
                    })
                });
            lower_points
                .into_iter()
                .tuple_windows()
                .for_each(|(from, to)| {
                    ctx.draw(&Line {
                        x1: from.x,
                        x2: to.x,
                        y1: from.y,
                        y2: to.y,
                        color: Color::Green,
                    })
                });
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, area);
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
        Spans::from(""),
        Spans::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        Spans::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
