fn main() {
    let number = 3;

    if number < 5 {
    	println!("{} is less than 5", number);
    } else {
    	println!("{} is not less than 5", number);
    }

    if number != 0 {
    	println!("{} is not zero", number);
    }

    let number = 17;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
