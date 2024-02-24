#[derive(Clone, Debug, Default)]
pub struct Time {
  pub instant: Option<std::time::Instant>,
}

impl Time {
  pub fn now() -> Self {
    Self {
      instant: Some(std::time::Instant::now()),
    }
  }

  #[inline]
  pub fn merge(start: &Self, end: &Self) -> Option<std::time::Duration> {
    match (start.instant, end.instant) {
      (Some(start), Some(end)) => Some(end.duration_since(start)),
      _ => None,
    }
  }
}

impl std::fmt::Display for Time {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
