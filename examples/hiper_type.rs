use core::panic;
use std::result::Result as stdResult;

// 使用例
// cargo run --example hiper_type
fn main() {
    // ニュータイプパターン
    // 既存型をラッパーする形で別型として扱う
    // 型安全性と抽象化を助ける
    #[derive(Debug)]
    struct Meter(u32);
    #[derive(Debug)]
    struct CentiMeter(u32);

    impl From<CentiMeter> for Meter {
        fn from(cm: CentiMeter) -> Meter {
            assert!(cm.0 % 100 == 0);
            Meter(cm.0 / 100)
        }
    }

    let ten_meter: Meter = Meter(10);
    let two_hundred_centimeter: CentiMeter = CentiMeter(200);

    fn rectangle_perimeter_length(width: Meter, height: Meter) -> Meter {
        Meter((width.0 + height.0) * 2)
    }

    println!(
        "Rectangle perimeter length is {} meter",
        rectangle_perimeter_length(ten_meter, Meter::from(two_hundred_centimeter)).0
    );


    // 型エイリアス: ある型の名前だけを変えて別typeとする
    // usecase:
    //   型をより具体的な名前にしたい。
    //   長い名前の型を繰り返し利用するのを減らしたい。
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    // 同じ型として扱われるので、足し算ができる
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        println!("Hello, world!");
    }
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
    takes_long_type(f);
    let hi: Thunk = returns_long_type();

    // Result<T> = std::result::Result<T, std::io::Error> で定義しておくと、Result記載時に毎回エラーを書かなくて良いので便利
    type Result<T> = stdResult<T, std::io::Error>;

    pub trait Hello {
        // 冗長なhello
        fn redundant_hello() -> stdResult<String, std::io::Error>;
        // 簡潔なhello
        fn concise_hello() -> Result<String>;
    }

    // never型: 絶対に返らない！
    // 関数が型を返さないときに利用する。! で定義できる
    // usecase:
    //   panicする関数
    //   ループを無限に回す関数
    //   let x = match ... の「型は必ず統一する」というルールを無視できる = 任意の型に型矯正できる
    fn bar() -> ! {
        panic!("This function never returns!");
    }

    fn answer() -> Result<i32> {
        Ok(42)
    }
    let guess: i32 = match answer() {
        Ok(num) => num,
        Err(_) => panic!(),
    };
}
