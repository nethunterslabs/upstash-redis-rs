use crate::cmd;

cmd! {
  /// Get the value of a key.
  GET,
  Option<String>;
  key
}
