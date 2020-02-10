struct User {
	first_name: String,
	email: String,
	active: bool,
	sign_in_count: i32
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

//-- Unit-like structs can be useful in situations in which you need to implement a trait 
//-- on some type but don’t have any data that you want to store in the type itself..
struct FixThis();

fn main() {

    let u1 = User {
    	first_name: String::from("Bobby"),
    	email: String::from("bobby@gmail.com"),
    	active: true,
    	sign_in_count: 1
    };

    println!("u1: {} {} {} {}", u1.first_name, u1.email, u1.active, u1.sign_in_count);

    let mut u2 = create_user("Fischer".to_string(), "Fischer@gmail.com".to_string());
    
    u2.active = false;

    println!("u2: {} {} {} {}", u2.first_name, u2.email, u2.active, u2.sign_in_count);

    let u3 = User {
    	email: "Fischer@hotmail.com".to_string(),
    	..u2
    };

    println!("u3: {} {} {} {}", u3.first_name, u3.email, u3.active, u3.sign_in_count);

    let mut white = Color(255, 0, 255);

    white.1 = 255;

    let Color(r, g, b) = white;

    println!("Color: {} {} {}", r, g, b);

    let pt = Point(1, 2, 3);

    println!("Point: {} {} {}", pt.0, pt.1, pt.2);

    let _todo = FixThis();
}

fn create_user(first_name: String, email: String) -> User {
	User {
		first_name,
		email,
		active: true,
		sign_in_count: 0
	}
}
