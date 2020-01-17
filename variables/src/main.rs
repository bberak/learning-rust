fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: i32 = 100_000;

    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    println!("Various number literals: {} {} {} {} {}", 42, 0xff, 0o77, 0b1111_1111, b'~');

    let a: u8 = 252;

    let b: u8 = 3;

    let c = a + b;

    println!("{} {} {}", a, b, c);

    let fp32: f32 = 2.123456789_10_11_12;

    let fp64: f64 = 2.123456789_11_13_15_17_19_21_23;

    println!("fp32: {}, fp64: {}", fp32, fp64);

    let remainder = 43 % 5;

    println!("Remainder: {}", remainder);

    let d = 'z';
    let e = b'z';
    let f = 'ℤ';
    let heart_eyed_cat = '😻';

   	println!("Character types: {} {} {} {}", d, e, f, heart_eyed_cat);

   	let tup = (500, 6.4, 1);

   	let (x, y, z) = tup;

   	println!("Tup: {} {} {}, {} {} {}", x, y, z, tup.0, tup.1, tup.2);

   	let _arr = [7, 7, 7, 7, 7];
   	let _arr = [7; 5];
   	let arr: [i32; 5] = [1, 2, 3, 4, 5];
   	let [first, _, _, _, _] = arr;

   	println!("First item in arr: {}", first);
   	println!("Second item in arr: {}", arr[1]);

}
