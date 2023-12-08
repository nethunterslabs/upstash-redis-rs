use crate::cmd;

cmd! {
  /// Increment the integer value of a key by the given amount.
  INCRBY,
  usize;
  key,
  increment
}
