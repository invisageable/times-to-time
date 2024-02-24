use super::time::Time;
use super::unit::Unit;

/// A [`Timer`] representation.
//
#[derive(Clone, Debug, Default)]
pub struct Timer {
  /// A start time.
  pub maybe_time_start: Option<Time>,
  /// A end time.
  pub maybe_time_end: Option<Time>,
}

impl Timer {
  /// Creates an new [`Timer`] instance.
  //
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  /// ...
  ///
  /// ## examples.
  ///
  /// ```
  /// use times_to_time_timer::timer::Timer;
  ///
  /// let mut timer = Timer::new();
  ///
  /// timer.start();
  /// println!("{}", timer.maybe_time_start.unwrap());
  /// ```
  //
  #[inline]
  pub fn start(&mut self) {
    self.maybe_time_start = Some(Time::now());
  }

  /// ...
  ///
  /// ## examples.
  ///
  /// ```
  /// use times_to_time_timer::timer::Timer;
  ///
  /// let mut timer = Timer::new();
  ///
  /// timer.start();
  /// timer.sleep(100);
  /// timer.end();
  /// println!("{}", timer.maybe_time_end.unwrap());
  /// ```
  //
  #[inline]
  pub fn end(&mut self) {
    self.maybe_time_end = Some(Time::now());
  }

  /// ...
  //
  #[inline]
  pub fn sleep(&mut self, millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
  }

  /// ...
  //
  #[inline]
  pub fn reset(&mut self) {
    self.maybe_time_start = None;
    self.maybe_time_end = None;
  }

  /// Returns an optional [`std::time::Duration`] instance.
  //
  pub fn duration(&self) -> Option<std::time::Duration> {
    match (self.maybe_time_start.as_ref(), self.maybe_time_end.as_ref()) {
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

#[cfg(test)]
mod test {
  use super::Timer;

  #[test]
  fn should_make_timer() {
    let timer = Timer::new();

    assert!(timer.maybe_time_start == None);
    assert!(timer.maybe_time_end == None);
  }
}
