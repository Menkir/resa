fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("v[0] is: {}", v[0]);
    const C: i32 = 2;
    static CONST_REF: &'static mut i32 = &mut C;
}