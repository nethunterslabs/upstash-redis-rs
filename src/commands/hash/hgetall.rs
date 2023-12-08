use crate::cmd;

cmd! {
  /// Get all the fields and values in a hash.
  HGETALL,
  Vec<String>;
  key
}
