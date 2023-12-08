use crate::cmd;

cmd! {
  /// Add one or more members to a set.
  SADD,
  usize;
  key,
  member
}
