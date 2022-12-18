use crossterm::event::{KeyCode, KeyEvent};
use nalgebra::Point2;
use rand::Rng;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    pub title: &'a str,

    pub input_mode: InputMode,
    pub input: String,

    pub points: Option<Vec<Point2<f64>>>,
    pub x_bounds: [f64; 2],
    pub y_bounds: [f64; 2],

    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> App<'a> {
        let point_amount = 20;

        App {
            title,
            input_mode: InputMode::Normal,
            input: String::new(),
            points: None,
            x_bounds,
            y_bounds,
            should_quit: false,
        }
    }

    fn generate_points(&self, point_amount: usize) -> Vec<Point2<f64>> {
        let mut points = vec![];

        for _ in 0..point_amount {
            let x = rand::thread_rng().gen_range(self.x_bounds[0]..=self.x_bounds[1]);
            let y = rand::thread_rng().gen_range(self.y_bounds[0]..=self.y_bounds[1]);
            let point = Point2::new(x, y);
            points.push(point);
        }
        points
    }

    pub fn on_key(&mut self, key: KeyEvent) {
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
                    // TODO: Set point_amount right here.
                    self.points = Some(self.generate_points(self.input.parse::<usize>().unwrap()));
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
                _ => {}
            },
        }
    }
}
