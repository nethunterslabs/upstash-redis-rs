use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {
    /// Removes the specified members from a stream.
    XDEL,
    usize;
    key,
    id
}

impl XdelCommand {
    pub fn add_id<S: Serialize>(&mut self, id: S) -> Result<&mut Self> {
        self.set_options(id)
    }
}
