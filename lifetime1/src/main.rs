fn main() {
    let x: i32 = 5;         // ------------+---- 'b
                            //             |
    let r: &i32 = &x;       // --+--- 'a   |
                            //   |         |
    println!("r: {}", r)    //   |         |
}                           // ------------+
