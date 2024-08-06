use std::fmt::Error;

mod aws;
// ANCHOR: action
pub enum Action {
    Tick,
    Increment,
    Decrement,
    Quit,
    None,
}
// ANCHOR_END: action

// ANCHOR: application
/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
    pub parameter_store_names: Vec<String>
}
// ANCHOR_END: application

// ANCHOR: application_impl
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn setup_parameter_stores(&mut self){
        match aws::parameter_store::fetch_ps().await   {
            Ok(res) => {
                for list in res.iter() {
                    match &list {
                        Some(name) => self.parameter_store_names.push(name.clone()),
                        _ => panic!("error")
                    }
                }
            }
            _ => panic!("Error")
        };
    }


    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
// ANCHOR_END: application_impl

// ANCHOR: application_test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
// ANCHOR_END: application_test
