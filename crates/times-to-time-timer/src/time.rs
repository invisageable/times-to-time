/// A time representation.
//
#[derive(Clone, Debug, Default)]
pub struct Time {
  pub instant: Option<std::time::Instant>,
}

impl Time {
  /// Creates a new [`Time`] instance.
  //
  #[inline]
  pub fn now() -> Self {
    Self {
      instant: Some(std::time::Instant::now()),
    }
  }

  /// Merges times and then returns an optional [`std::time::Duration`]
  /// instance.
  //
  #[inline]
  pub fn merge(start: &Self, end: &Self) -> Option<std::time::Duration> {
    match (start.instant, end.instant) {
      (Some(start), Some(end)) => Some(end.duration_since(start)),
      _ => None,
    }
  }
}
