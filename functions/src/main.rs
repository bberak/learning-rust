fn main() {
    hello();
    world("w0rd!!".to_string());
    another_function(32, 64.0);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("five returns {}", five());

    println!("6 squared is {}", sqr(6));
}


fn hello() {
	println!("Hello, ");
}

fn world(x: String) {
	println!("{}", x);
}

fn another_function(x: i32, y: f32) {
	println!("{} {}", x, y);
}

fn five() -> i32 {
	println!("five() has been called");
	5
}

fn sqr(x: i32) -> i32 {
	//-- "return" is optional, so is a semicolon at the end of the line..
	return x * x
}