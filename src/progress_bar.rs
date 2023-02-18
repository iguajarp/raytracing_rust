use std::io::Write;

pub fn progress_bar(n: usize, total: usize, bar_size: usize) {
  let progress = n as f32 / total as f32;
  print!(
      "[{}{}] {}%\r",
      "=".repeat((progress * bar_size as f32) as usize),
      " ".repeat(bar_size - (progress * bar_size as f32) as usize),
      (progress * 100.0) as i32
  );
  
  if progress == 1.0 {
    println!();
  }
  std::io::stdout().flush().unwrap();

}