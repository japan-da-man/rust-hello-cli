// 使用例
// cargo run --example hiper_function_and_closure
fn main() {
    // 関数ポインタ: 何らかの関数が特定の関数を渡す
    // バイナリの関数を指し示す関数
    // usecase: C言語などRust以外の言語とやりとりするときはclosureがないのでこれを使う
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // クロージャを返す方法
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    // これはコンパイルエラー
    // 正しくは以下
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    let add_one = returns_closure();

    println!("closure: {}", add_one(5));

}
