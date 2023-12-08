use crate::cmd;

cmd! {
  /// Decrement the integer value of a key by one.
  DECR,
  usize;
  key
}
