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

    // 2つのスレッドから1つのレシーバーにセンドする
    let (txx, rxx) = mpsc::channel();
    // 1つ目のスレッドから送信
    let tx1 = mpsc::Sender::clone(&txx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });
    // 2つ目のスレッドから送信
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            txx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });
    for received in rxx {
        println!("Got {}", received);
    };
}
