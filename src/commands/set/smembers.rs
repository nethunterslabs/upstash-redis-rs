use crate::cmd;

cmd! {
  /// Get all the members in a set.
  SMEMBERS,
  Vec<String>;
  key
}
