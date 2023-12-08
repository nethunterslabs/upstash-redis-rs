use crate::cmd;

cmd! {
  /// Append a value to a key.
  APPEND,
  usize;
  key,
  value
}
