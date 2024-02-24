use times_to_time_timer::timer::Timer;
use times_to_time_timer::unit::Unit;

fn main() {
  let mut timer = Timer::new();

  timer.start();
  std::thread::sleep(std::time::Duration::from_millis(100));
  timer.end();

  let Some(seconds) = timer.duration_in_unit(Unit::S) else {
    std::process::exit(1);
  };

  println!("time: {seconds:.6}s");
}
