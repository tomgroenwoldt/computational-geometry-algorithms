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
    pub step: usize,
    pub max_steps: Option<usize>,
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
            step: 0,
            max_steps: None,
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
                    Ok(())
                }
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    Ok(())
                }
                _ => Ok(()),
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    self.step = 0;
                    self.point_amount = Some(self.input.parse::<usize>()?);
                    self.generate_points();
                    self.algorithm.calculate();
                    self.max_steps = Some(self.algorithm.maximum_step_count);
                    Ok(())
                }
                KeyCode::Char(c) => {
                    if c.is_ascii_digit() {
                        self.input.push(c);
                    }
                    Ok(())
                }
                KeyCode::Backspace => {
                    self.input.pop();
                    Ok(())
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    Ok(())
                }
                KeyCode::Right => {
                    if let Some(max_steps) = self.max_steps {
                        if self.step < max_steps - 1 {
                            self.step += 1;
                        }
                    }
                    Ok(())
                }
                KeyCode::Left => {
                    if self.step > 0 {
                        self.step -= 1;
                    }
                    Ok(())
                }
                _ => Ok(()),
            },
        }
    }
}
