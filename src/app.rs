pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
        }
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        }
    }

    pub fn on_tick(&mut self) {
        // TODO: Update progress
    }
}
