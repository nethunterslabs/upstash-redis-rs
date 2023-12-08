use crate::cmd;

cmd! {
  /// Get the length of the value stored in a key.
  STRLEN,
  usize;
  key
}
