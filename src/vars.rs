pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_00;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();
    // 整数
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);
    let i1 = 3;
    let f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // ポインタ
    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("value of y is: {}", y);
    {
        let y = 0;
        println!("value of y is: {}", y);
    }
    println!("value of y is: {}", y);

    // タプル型
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value of t1 is: {} {} {}", x, y, z);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[0], a2[0]);

    // 文字列スライス
    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    println!("Static address of s1 {:p}", &s1);
    println!("Static address of s2 {:p}", &s1);
    println!("Static memory address of s1 {:?}", s1.as_ptr());
    println!("Static memory address of s2 {:?}", s2.as_ptr());
    println!("Static memory length of s1 {}", &s1.len());
    println!("Static memory length of s2 {}", &s2.len());

    // string型
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Static address of s1 {:p}", &s1);
    println!("Static address of s2 {:p}", &s2);

    // heap領域に格納されている文字列の先頭のアドレス
    println!("Heap memory address of s1 {:p}", s1.as_ptr());
    println!("Heap memory address of s2 {:p}", s2.as_ptr());

    // heap領域に格納されている文字列の長さ
    println!("Len of s1 {}", s1.len());
    println!("Len of s2 {}", s2.len());

    // heap領域に実データ保存できる容量（Rust側が勝手に調整してくれる）
    println!("Capacity of s1 {}", s1.capacity());
    println!("Capacity of s2 {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);
    println!("Len of s1 {}", s1.len());
    println!("Len of s2 {}", s2.len());
    println!("Capacity of s1 {}", s1.capacity());
    println!("Capacity of s2 {}", s2.capacity());
}
