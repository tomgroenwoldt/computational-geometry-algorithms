use nalgebra::Point2;
use tui::{
    backend::Backend,
    layout::Rect,
    style::Color,
    symbols,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders,
    },
    Frame,
};

use crate::app::App;

/// # Different drawing methods.
/// Defines how the steps of an algorithm
/// should be rendered.
pub enum DrawMethod {
    Edge,
}

/// Every algorithm implementing this trait is able
/// to render within the general ui.
pub trait Algorithm {
    fn draw_algorithm<B>(&self, f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend,
    {
        let canvas = Canvas::default()
            .block(Block::default().title("Algorithm").borders(Borders::ALL))
            .paint(|ctx| {
                let points = self.get_points();

                // If there are no points, don't render anything.
                if points.is_empty() {
                    return;
                }

                let steps = self.get_steps();

                // Draw steps calculated by algorithm.
                match self.get_draw_method() {
                    DrawMethod::Edge => steps[app.step].iter().for_each(|line| ctx.draw(line)),
                }
                ctx.layer();

                // Draw initial points after steps to prevent overdrawing them.
                ctx.draw(&Points {
                    coords: &points
                        .iter()
                        .map(|point| (point.x, point.y))
                        .collect::<Vec<_>>(),
                    color: Color::Red,
                })
            })
            .marker(symbols::Marker::Braille)
            .x_bounds(app.x_bounds)
            .y_bounds(app.y_bounds);
        f.render_widget(canvas, area);
    }

    /// Get the initial point set of the algorithm.
    fn get_points(&self) -> Vec<Point2<f64>>;

    /// Get all computed steps. Every step is stored as a vector of
    /// lines. This is convenient because we can iterate through the algorithm
    /// steps after a single computation.
    fn get_steps(&self) -> Vec<Vec<Line>>;

    fn get_draw_method(&self) -> DrawMethod;
}
