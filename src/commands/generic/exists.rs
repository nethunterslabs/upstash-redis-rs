use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {
    /// Determine if a key exists.
    EXISTS,
    usize;
    key
}

impl ExistsCommand {
    pub fn add_key<S: Serialize>(&mut self, key: S) -> Result<&mut Self> {
        self.set_options(key)
    }

    pub fn add_keys<S>(&mut self, keys: S) -> Result<&mut Self>
    where
        S: IntoIterator,
        <S as IntoIterator>::Item: Serialize,
    {
        for key in keys {
            self.set_options(key)?;
        }
        Ok(self)
    }
}
