use crate::{cmd, Command, Result};

cmd! {
    /// Remove and return one or multiple random members from a set.
    SPOP,
    Option<String>;
    key
}

impl SpopCommand {
    pub fn add_count(&mut self, count: usize) -> Result<&mut Self> {
        self.set_options(count)
    }
}
