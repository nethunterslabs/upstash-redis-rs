use crate::cmd;

cmd! {
  /// Increment the integer value of a key by one.
  INCR,
  usize;
  key
}
