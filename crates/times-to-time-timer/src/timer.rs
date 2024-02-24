use super::time::Time;
use super::unit::Unit;

/// A [`Timer`] representation.
//
#[derive(Clone, Debug, Default)]
pub struct Timer {
  /// A start time.
  pub maybe_time_start: Option<Time>,
  /// A end time.
  pub mabe_time_end: Option<Time>,
}

impl Timer {
  /// Creates an new [`Timer`] instance.
  //
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  /// ...
  //
  #[inline]
  pub fn start(&mut self) {
    self.maybe_time_start = Some(Time::now());
  }

  /// ...
  //
  #[inline]
  pub fn end(&mut self) {
    self.mabe_time_end = Some(Time::now());
  }

  /// ...
  //
  #[inline]
  pub fn reset(&mut self) {
    self.maybe_time_start = None;
    self.mabe_time_end = None;
  }

  /// Returns an optional [`std::time::Duration`] instance.
  //
  pub fn duration(&self) -> Option<std::time::Duration> {
    match (self.maybe_time_start.as_ref(), self.mabe_time_end.as_ref()) {
      (Some(start), Some(end)) => Time::merge(start, end),
      _ => None,
    }
  }

  /// ...
  //
  pub fn duration_in_unit<U: Into<Unit>>(&self, unit: U) -> Option<f64> {
    self
      .duration()
      .map(|duration| duration.as_nanos() as f64 / unit.into().as_factor())
  }
}

impl Drop for Timer {
  fn drop(&mut self) {
    self.reset();
  }
}
