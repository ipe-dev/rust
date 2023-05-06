use std::string;

pub fn run() {
    let s1 = String::from("hello");
    // 所有権がs2に移る
    let s2 = s1;
    // s1の所有権が移動しているのでs1にはアクセスできない
    // println!("{} {}", s1, s2);
    println!("{}", s2);

    // i32型はstack内でコピーされる
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    // 文字列スライスもstack内でコピーされる
    // 文字列スライスは実データがある静的領域のアドレスを「参照」しているだけなので、所有権の移動は起こらない
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    // sl1とsl2は静的領域の同じアドレスを参照してる
    println!("Static address of sl1 is: {:p}", sl1.as_ptr());
    println!("Static address of sl2 is: {:p}", sl2.as_ptr());
    // stack内のアドレスは別々
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    // Stringをdeep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    // ヒープ内の別々のアドレスにhelloが入る
    println!("Heap address of hello is: {:p}", s3.as_ptr());
    println!("Heap address of hello is: {:p}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap address of hello is: {:p}", s5.as_ptr());
    println!("Len of s5 is: {}", s5.len());
    println!("Capacity of s5 is: {}", s5.capacity());
    // 所有権がメソッドに移る
    take_ownership(s5);
    // 所有権がないのでs5にアクセスできない
    // println!("{}", s5);

    let s6 = String::from("hello");
    println!("Stack address of s6: {:p}", &s6);
    println!("Heap address of hello: {:p}", s6.as_ptr());
    println!("Len of s6: {}", s6.len());

    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7: {:p}", &s7);
    println!("Heap address of hello: {:p}", s7.as_ptr());
    println!("Len of s7: {}", s7.len());

    let s8 = String::from("本日は晴天なれども波高し");
    let len = calculate_length(&s8);
    println!("The length of '{}' is '{}' ", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // let s10 = String::from("hello");
    // let r1 = &s10;
    // 他でイミュータブルな参照をしている場合、ミュータブルな参照はできなくなる
    // let r2 = &mut s10;
    // println!("{} {} {}", s10, r1, r2);

    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    r1.push_str("oooo");
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}
fn take_ownership(s: String) {
    println!("Stack address of s is: {:p}", &s);
    println!("Heap address of hello is: {:p}", s.as_ptr());
    println!("Len of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());
    println!("{}", s);
}
fn take_giveback_ownership(s: String) -> String {
    s
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(s: &mut String) {
    s.push_str("_world");
}
