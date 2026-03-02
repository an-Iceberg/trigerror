pub struct Trigerror
{
  interfaces: Vec<String>,
  filters: Option<Vec<String>>,
  count_before: u32,
  count_after: u32,
  /// Time in miliseconds
  time_before: f32,
  /// Time in miliseconds
  time_after: f32,
  retrigger: bool,
}
