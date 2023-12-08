use crate::cmd;

cmd! {
  /// Get the value of a hash field.
  HGET,
  Option<String>;
  key,
  field
}
