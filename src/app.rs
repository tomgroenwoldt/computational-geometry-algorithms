use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use nalgebra::Point2;
use rand::Rng;

use crate::algorithms::graham_scan::GrahamScan;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    pub title: &'a str,

    pub input_mode: InputMode,
    pub input: String,

    pub algorithm: GrahamScan,
    pub upper_step: usize,
    pub lower_step: usize,
    pub point_amount: Option<usize>,

    pub x_bounds: [f64; 2],
    pub y_bounds: [f64; 2],

    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> App<'a> {
        App {
            title,
            input_mode: InputMode::Normal,
            input: String::new(),
            algorithm: GrahamScan::new(),
            upper_step: 0,
            lower_step: 0,
            point_amount: None,
            x_bounds,
            y_bounds,
            should_quit: false,
        }
    }

    /// Generates points in random locations bounded by the App structs bounds and
    /// passes them to its defined algorithm.
    fn generate_points(&mut self) {
        let mut points = vec![];

        if let Some(point_amount) = self.point_amount {
            for _ in 0..point_amount {
                let x = rand::thread_rng().gen_range(self.x_bounds[0]..=self.x_bounds[1]);
                let y = rand::thread_rng().gen_range(self.y_bounds[0]..=self.y_bounds[1]);
                let point = Point2::new(x, y);
                points.push(point);
            }
        }
        self.algorithm.points = points;
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Result<(), Error> {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('i') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    self.upper_step = 0;
                    self.lower_step = 0;
                    self.point_amount = Some(self.input.parse::<usize>()?);
                    self.generate_points();
                    self.algorithm.calculate();
                }
                KeyCode::Char(c) => {
                    if c.to_digit(10).is_some() {
                        self.input.push(c);
                    }
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Right => {
                    if self.upper_step + self.lower_step
                        < self.algorithm.upper_steps.len() + self.algorithm.lower_steps.len() - 1
                    {
                        if self.upper_step < self.algorithm.upper_steps.len() - 1 {
                            self.upper_step += 1;
                        } else if self.lower_step < self.algorithm.lower_steps.len() - 1 {
                            self.lower_step += 1;
                        }
                    }
                }
                KeyCode::Left => {
                    if self.upper_step + self.lower_step > 0 {
                        if self.lower_step > 0 {
                            self.lower_step -= 1;
                        } else {
                            self.upper_step -= 1;
                        }
                    }
                }
                _ => {}
            },
        }
        Ok(())
    }
}
