use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
  let data = Arc::new(Mutex::new(0));
  let mut threads = vec![];
  for thread_id in 0..3 {
    println!("スレッド (ID: {}) を開始します", thread_id);
    let data = data.clone();
    threads.push(thread::spawn(move || {
      for _ in 0..100000 {
        *data.lock().unwrap() += 1;
      }
      println!("スレッド (ID: {}) が終了しました", thread_id);
    }));
  }

  for t in threads {
    t.join().unwrap();
  }

  println!("合計: {}", *data.lock().unwrap());
}
