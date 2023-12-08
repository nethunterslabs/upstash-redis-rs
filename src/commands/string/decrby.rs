use crate::cmd;

cmd! {
  /// Decrement the integer value of a key by the given number.
  DECRBY,
  usize;
  key,
  decrement
}
