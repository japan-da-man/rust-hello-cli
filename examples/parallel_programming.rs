use std::thread;
use std::time::Duration;
use std::sync::mpsc;

// 使用例
// cargo run --example parallel_programming
fn main() {
    // thread::spawnで新しいスレッドを生成
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // threadの終了を待つ

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // moveクロージャーにより所有権を移動
        println!("{:?}", v);
    });
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 以下はコンパイルエラー: すでに チャンネル に値と所有権を送っているため
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("{}", received);
}
