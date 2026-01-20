use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    // cargo run --example smartpointer

    // 借用+アスタリスク * で参照先を見る
    let a = vec![1, 2, 3];
    let borrowed_a = &a;
    let b = vec![1, 2, 3];
    println!("equality: {}", *borrowed_a == b);

    let mut moved_a = a;
    let muttably_borrowed_a = &mut moved_a;
    *muttably_borrowed_a = vec![4, 5, 6];
    println!("mutably_borrowed_a: {:?}", *muttably_borrowed_a);   

    // スマートポインタとは
    // 参照に似た概念。参照と違って、アドレスだけでなく、データ（=所有権）を持つ
    // String や Vecがスマートポインタに当てはまる
    // Deref と Drop を持つ構造体
    // 以下に、スマートポインタの例を説明する

    // Box<T>
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Rc<T>
    // 参照カウンタ: 参照がなくなったときにdrop
    // 1つの変数を複数で参照し合う場合に必要な概念
    enum list {
        Cons(i32, Rc<list>),
        Nil,
    }

    use list::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // a 生成後のカウント: {}
    println!("count after create a: {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    // b 生成後のカウント: {}
    println!("count after create b: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        // c 生成後のカウント: {}
        println!("count after create c: {}", Rc::strong_count(&a));
    }
    // c がスコープを抜けた後のカウント: {}
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));

    // RefCell
    // 実行時に借用規制を行う
    // Rcと組み合わせることで、可変なデータに複数の所有者を持たせる
}
