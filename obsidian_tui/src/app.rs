use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Exiting,
}

pub struct App {
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
}

impl App {
    pub fn new() -> App {
        App {
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
