fn main() {
	let mut counter = 0;
    
    let result = loop {
        println!("again! {}", counter);
        counter = counter + 1;

        if counter > 10 {
        	break counter;
        }
    };

    println!("result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

 	let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for n in a.iter() {
    	println!("n is {}", n);
    }

    for n in (0..11).rev() {
    	if n == 0 { 
    		println!("BLASTOFF!"); 
    	} else { 
    		println!("countdown: {}", n); 
    	}
    }
}