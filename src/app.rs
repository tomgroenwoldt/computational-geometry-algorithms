use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use nalgebra::Point2;
use rand::Rng;

use crate::algorithms::{algorithm::AlgorithmWrapper, graham_scan::GrahamScan};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct TabsState {
    pub tabs: Vec<Tab>,
    pub index: usize,
}

pub struct Tab {
    pub algorithm: AlgorithmWrapper,
    pub step: usize,
    pub max_steps: Option<usize>,
    pub point_amount: Option<usize>,
}

impl Tab {
    fn new(algorithm: AlgorithmWrapper) -> Self {
        Tab {
            algorithm,
            step: 0,
            max_steps: None,
            point_amount: None,
        }
    }
}

impl TabsState {
    pub fn new(tabs: Vec<Tab>) -> TabsState {
        TabsState { tabs, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tab_state: TabsState,

    pub input_mode: InputMode,
    pub input: String,

    pub x_bounds: [f64; 2],
    pub y_bounds: [f64; 2],

    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> App<'a> {
        App {
            title,
            tab_state: TabsState::new(vec![
                Tab::new(AlgorithmWrapper::GrahamScan(GrahamScan::new())),
                Tab::new(AlgorithmWrapper::GrahamScan(GrahamScan::new())),
            ]),
            input_mode: InputMode::Normal,
            input: String::new(),
            x_bounds,
            y_bounds,
            should_quit: false,
        }
    }

    pub fn get_current_tab(&self) -> &Tab {
        if let Some(tab) = &self.tab_state.tabs.get(self.tab_state.index) {
            return tab;
        }
        self.tab_state
            .tabs
            .get(0)
            .expect("The application has no tabs open.")
    }

    pub fn get_current_tab_mut(&mut self) -> &mut Tab {
        &mut self.tab_state.tabs[self.tab_state.index]
    }

    pub fn reset_tab(&mut self) -> Result<(), Error> {
        self.get_current_tab_mut().step = 0;
        self.get_current_tab_mut().point_amount = Some(self.input.parse::<usize>()?);
        Ok(())
    }

    /// Generates points in random locations bounded by the App structs bounds and
    /// passes them to its defined algorithm.
    fn generate_points(&mut self) {
        let mut points = vec![];

        if let Some(point_amount) = self.get_current_tab().point_amount {
            for _ in 0..point_amount {
                let x = rand::thread_rng().gen_range(self.x_bounds[0]..=self.x_bounds[1]);
                let y = rand::thread_rng().gen_range(self.y_bounds[0]..=self.y_bounds[1]);
                let point = Point2::new(x, y);
                points.push(point);
            }
        }
        self.get_current_tab_mut().algorithm.set_points(points);
    }

    pub fn setup_tab(&mut self) {
        self.get_current_tab_mut().algorithm.calculate();
        self.get_current_tab_mut().max_steps =
            Some(self.get_current_tab().algorithm.get_maximum_step_count());
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Result<(), Error> {
        match key.code {
            KeyCode::Right => {
                let tab = self.get_current_tab_mut();
                if let Some(max_steps) = tab.max_steps {
                    if tab.step < max_steps - 1 {
                        tab.step += 1;
                    }
                }
            }
            KeyCode::Left => {
                let tab = self.get_current_tab_mut();
                if tab.step > 0 {
                    tab.step -= 1;
                }
            }
            KeyCode::Tab => {
                self.tab_state.next();
            }
            KeyCode::BackTab => {
                self.tab_state.previous();
            }
            _ => {}
        }
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
                    self.reset_tab()?;
                    self.generate_points();
                    self.setup_tab();
                }
                KeyCode::Char(c) => {
                    if c.is_ascii_digit() {
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
        };
        Ok(())
    }
}
