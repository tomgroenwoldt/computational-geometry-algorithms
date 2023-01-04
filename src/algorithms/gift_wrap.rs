use itertools::Itertools;
use nalgebra::{Point2, Vector2};
use tui::{style::Color, widgets::canvas::Line};

use super::algorithm::{Algorithm, DrawMethod};

pub struct GiftWrap {
    pub step_count: usize,
    // Later passed to app.
    pub maximum_step_count: usize,
    pub current_point_amount: usize,
    pub points: Vec<Point2<f64>>,
    pub steps: Vec<Vec<Point2<f64>>>,
}

enum Step {
    Addition,
    Deletion,
}

enum Orientation {
    Upper,
    Lower,
}

impl GiftWrap {
    pub fn new() -> Self {
        GiftWrap {
            step_count: 0,
            maximum_step_count: 0,
            current_point_amount: 0,
            points: vec![],
            steps: vec![],
        }
    }

    fn get_orientation(
        point_one: Point2<f64>,
        point_two: Point2<f64>,
        point_three: Point2<f64>,
    ) -> f64 {
        (point_two.y - point_one.y) * (point_three.x - point_two.x) * (point_three.y - point_two.y)
    }

    fn add_step(&mut self, step: Step, point: Option<Point2<f64>>, orientation: Orientation) {}

    pub fn calculate(&mut self) {
        self.steps = vec![];
        self.maximum_step_count = 0;
        self.step_count = 0;
        self.current_point_amount = 0;

        let mut points = self.points.clone();

        if points.len() < 3 {
            return;
        }

        // Get the left most point. On equal x values take the lower point.
        let mut minimum_x = points[0].x;
        let mut index_of_minimum = 0;
        points.iter().enumerate().for_each(|(i, point)| {
            let x = point.x;
            let y = point.y;
            if x < minimum_x || minimum_x == x && y < points[index_of_minimum].y {
                minimum_x = x;
                index_of_minimum = i;
            }
        });
    }
}

impl Algorithm for GiftWrap {
    fn get_title(&self) -> &str {
        "Gift wrap"
    }

    fn get_points(&self) -> &Vec<Point2<f64>> {
        &self.points
    }

    fn set_points(&mut self, points: Vec<Point2<f64>>) {
        self.points = points;
    }

    fn get_steps(&self) -> Vec<Vec<Line>> {
        let lines = self
            .steps
            .iter()
            .map(|upper_step| {
                upper_step
                    .iter()
                    .tuple_windows()
                    .map(|(from, to)| Line {
                        x1: from.x,
                        x2: to.x,
                        y1: from.y,
                        y2: to.y,
                        color: Color::Blue,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        lines
    }

    fn get_draw_method(&self) -> DrawMethod {
        DrawMethod::Edge
    }
}
