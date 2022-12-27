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

use super::graham_scan::GrahamScan;

/// # Different drawing methods.
/// Defines how the steps of an algorithm
/// should be rendered.
pub enum DrawMethod {
    Edge,
}

/// This is a wrapper for usage in a vector.
/// Because we are not allowed to use trait objects,
/// we need this abstraction.
pub enum AlgorithmWrapper {
    GrahamScan(GrahamScan),
}

impl AlgorithmWrapper {
    /// Sets the initial point set the algorithm works with.
    pub fn set_points(&mut self, points: Vec<Point2<f64>>) {
        match self {
            AlgorithmWrapper::GrahamScan(algorithm) => algorithm.set_points(points),
        }
    }

    /// Executes the algorithm.
    pub fn calculate(&mut self) {
        match self {
            AlgorithmWrapper::GrahamScan(algorithm) => algorithm.calculate(),
        }
    }

    /// Retrieves the maximum step count needed for the application
    /// to limit the user set step size.
    pub fn get_maximum_step_count(&self) -> usize {
        match self {
            AlgorithmWrapper::GrahamScan(algorithm) => algorithm.maximum_step_count,
        }
    }

    /// Draws the algorithm to the given area in the frame of the app.
    pub fn draw<B>(&self, f: &mut Frame<B>, area: Rect, app: &App)
    where
        B: Backend,
    {
        match self {
            AlgorithmWrapper::GrahamScan(algorithm) => algorithm.draw(f, area, app),
        }
    }
}

pub trait Algorithm {
    fn get_title(&self) -> &str;

    /// Get the initial point set of the algorithm.
    fn get_points(&self) -> &Vec<Point2<f64>>;

    /// Set the initial point set of the algorithm.
    fn set_points(&mut self, points: Vec<Point2<f64>>);

    /// Get all computed steps. Every step is stored as a vector of
    /// lines. This is convenient because we can iterate through the algorithm
    /// steps after a single computation.
    fn get_steps(&self) -> Vec<Vec<Line>>;

    fn get_draw_method(&self) -> DrawMethod;

    // TODO: If needed this draw method could be shifted to the individual algorithms
    // to enable customized rendering easily.
    fn draw<B>(&self, f: &mut Frame<B>, area: Rect, app: &App)
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
                    DrawMethod::Edge => steps[app.get_current_tab().step]
                        .iter()
                        .for_each(|line| ctx.draw(line)),
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
}
