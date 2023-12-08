use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {
    /// Removes the specified fields from the hash stored at key. Specified fields that do not exist within this hash are ignored. If key does not exist, it is treated as an empty hash and this command returns 0.
    HDEL,
    usize;
    key,
    field
}

impl HdelCommand {
    pub fn add_field<S: Serialize>(&mut self, field: S) -> Result<&mut Self> {
        self.set_options(field)
    }
}
