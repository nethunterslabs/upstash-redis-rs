use crate::cmd;

cmd! {
  /// Increment the value of a key by the given floating point amount.
  INCRBYFLOAT,
  String;
  key,
  increment
}
