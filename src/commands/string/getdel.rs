use crate::cmd;

cmd! {
  /// Get a key after deleting it.
  GETDEL,
  Option<String>;
  key
}
