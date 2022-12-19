use nalgebra::{Point2, Vector2};

pub struct GrahamScan {
    pub step_count: usize,
    pub current_point_amount: usize,
    pub points: Vec<Point2<f64>>,
    pub upper_steps: Vec<Vec<Point2<f64>>>,
    pub lower_steps: Vec<Vec<Point2<f64>>>,
}

enum Step {
    Addition,
    Deletion,
}

enum Orientation {
    Upper,
    Lower,
}

impl GrahamScan {
    pub fn new() -> Self {
        GrahamScan {
            step_count: 0,
            current_point_amount: 0,
            points: vec![],
            upper_steps: vec![],
            lower_steps: vec![],
        }
    }

    fn add_step(&mut self, step: Step, point: Option<Point2<f64>>, orientation: Orientation) {
        match orientation {
            Orientation::Upper => {
                self.upper_steps.push(vec![]);
                if self.step_count > 0 {
                    self.upper_steps[self.step_count] =
                        self.upper_steps[self.step_count - 1].clone();
                }

                match step {
                    Step::Addition => {
                        if let Some(point) = point {
                            self.upper_steps[self.step_count].push(point);
                            self.current_point_amount += 1;
                        }
                    }
                    Step::Deletion => {
                        self.upper_steps[self.step_count].remove(self.current_point_amount - 2);
                        self.current_point_amount -= 1;
                    }
                }
                self.step_count += 1;
            }
            Orientation::Lower => {
                self.lower_steps.push(vec![]);
                if self.step_count > 0 {
                    self.lower_steps[self.step_count] =
                        self.lower_steps[self.step_count - 1].clone();
                }

                match step {
                    Step::Addition => {
                        if let Some(point) = point {
                            self.lower_steps[self.step_count].push(point);
                            self.current_point_amount += 1;
                        }
                    }
                    Step::Deletion => {
                        self.lower_steps[self.step_count].remove(self.current_point_amount - 2);
                        self.current_point_amount -= 1;
                    }
                }
                self.step_count += 1;
            }
        }
    }

    fn new_cross_product(&mut self, orientation: Orientation) -> f64 {
        let (one, two, three) = match orientation {
            Orientation::Upper => (
                self.upper_steps[self.step_count - 1][self.current_point_amount - 3],
                self.upper_steps[self.step_count - 1][self.current_point_amount - 2],
                self.upper_steps[self.step_count - 1][self.current_point_amount - 1],
            ),
            Orientation::Lower => (
                self.lower_steps[self.step_count - 1][self.current_point_amount - 3],
                self.lower_steps[self.step_count - 1][self.current_point_amount - 2],
                self.lower_steps[self.step_count - 1][self.current_point_amount - 1],
            ),
        };
        let vector_first = Vector2::new(two.x - one.x, two.y - one.y);
        let vector_last = Vector2::new(three.x - two.x, three.y - two.y);
        vector_first.x * vector_last.y - vector_last.x * vector_first.y
    }

    pub fn calculate(&mut self) {
        self.upper_steps = vec![];
        self.lower_steps = vec![];
        self.step_count = 0;
        self.current_point_amount = 0;

        let mut points = self.points.clone();

        // Sort points lexicographically.
        points.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap());

        // Handle base cases.
        if points.len() < 3 {
            match points.len() {
                1 => {
                    self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Upper);
                    return;
                }
                2 => {
                    self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Upper);
                    self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Upper);
                    return;
                }
                _ => {}
            }
        }

        // Insert the first two points.
        self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Upper);
        self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Upper);

        for _ in 0..points.len() {
            self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Upper);
            while self.upper_steps[self.step_count - 1].len() > 2
                && self.new_cross_product(Orientation::Upper) > 0.0
            {
                self.add_step(Step::Deletion, None, Orientation::Upper);
            }
        }

        self.step_count = 0;
        self.current_point_amount = 0;

        // Sort points lexicographically.
        let mut points = self.points.clone();
        points.sort_by(|a, b| (a.x, a.y).partial_cmp(&(b.x, b.y)).unwrap());

        // Insert the first two points.
        self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Lower);
        self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Lower);

        for _ in 0..points.len() {
            self.add_step(Step::Addition, Some(points.remove(0)), Orientation::Lower);
            while self.lower_steps[self.step_count - 1].len() > 2
                && self.new_cross_product(Orientation::Lower) < 0.0
            {
                self.add_step(Step::Deletion, None, Orientation::Lower);
            }
        }
    }
}
