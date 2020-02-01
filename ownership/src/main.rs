fn main() {
	let s1 = String::from("s1");
	let s2 = s1; //-- s1 is moved to s2 (shallow copy). s1 is now invalid

	println!("s2: {}", s2);

	let s3 = String::from("s3");
	let s4 = s3.clone(); //-- s3 is cloned to s4 (deep copy). Both s3 and s4 are valid

	println!("s3: {}, s4: {}", s3, s4);

	let x  = 5;
	let y = x.clone(); //-- clone is optional, scalars are stored on the stack so are always deep copied

	println!("x: {}, y: {}", x, y);

	let pt1 = (3, 4);
	let mut pt2 = pt1; //-- tuples are also deep copied if they contain only scalars
	
	pt2.1 = 6;

	println!("pt1: {}.{}, y: {}.{}", pt1.0, pt1.1, pt2.0, pt2.1);

	let s5 = String::from("s5");

	takes_ownership(s5); //-- s5 can no longer be used in this scope, is has been moved to the  first arg in takes_ownership

	//-- line below will won't compile because we are using a variable (s5) that is out of scope
	// println!("s5: {}", s5);

	let z = 42;

	makes_copy(x);

	println!("z: {}", z); //-- z is still valid, so it is save to uze

	let s6 = gives_ownership();

	println!("s6: {}", s6);

	let s7 = String::from("s7");

	println!("s7: {}", s7);

	let s8 = takes_and_gives_back(s7); //-- s7 becomes invalid because it was moved

	println!("s8: {}", s8);

	let s9 = String::from("s9");

	println!("s9: {}", s9);

	let s10 = calculate_length(s9);

	println!("s10: {}, length: {}", s10.0, s10.1);

	let s11 = calculate_length(String::from("s11"));

	println!("s11: {}, length: {}", s11.0, s11.1);

	let s12 = String::from("s12");

	let s12_len = calculate_length_ref(&s12);

	println!("s12: {}, length: {}", s12, s12_len);

	let mut s13 = String::from("s13");

	let s13_len = change_and_calculate_length_ref(&mut s13);

	println!("s13: {}, length: {}", s13, s13_len);

	let mut s14 = String::from("s14");

	println!("s14: {}", s14);

	//-- Can only have one mutable reference to a variable
	//-- in scope at any one time
	{
		let r1 = &mut s14;

		r1.push_str("_r1");

		println!("r1: {}", r1);
	}

	let r2 = &mut s14;

	r2.push_str("_r2");

	println!("r2: {}", r2);

	let s15 = String::from("s15");

	//-- More than one immutable reference in a single scope is accepted
	//-- because a read-only operation cannot affect the underlying data
	let r3 = &s15;
	let r4 = &s15;

	println!("s15: {}, r3: {}, r4: {}", s15, r3, r4);

	//-- The rust compiler determines that a reference is out of scope when it is no longer being
	//-- used. So even though r5 and r6 below are in the same function scope - their usage, and therefore
	//-- their scopes don't overlap - and therefore these sequential mutations are determined to be safe
	let mut s16 = String::from("s16");

	println!("s16: {}", s16);

	let r5 = &mut s16;

	r5.push_str("_r5");

	println!("r5: {}", r5);

	let r6 = &mut s16;

	r6.push_str("_r6");

	println!("r6: {}", r6);

	let mut phrase = String::from("This is pretty esoteric..");

	let word = first_word(&phrase);

	phrase.clear();

	println!("phrase: {}", phrase);
	println!("The first word in the phrase ends at position {}", word);

	let phrase_2 = String::from("This is also pretty esoteric..");

	let word_2 = first_word_with_slicing(&phrase_2);

	println!("phrase_2: {}", phrase_2);
	println!("The first word in phrase_2 is '{}'", word_2);

	let noon = "noon";
	let oo = &noon[1..3];

	println!("{} is in {}", oo, noon);

	let phrase_3 = String::from("abcdefg 1234567");
	let phrase_4 = "1234567 abcdefg";

	let word_3 = better_first_word_with_slicing(&phrase_3[..]);
	let word_4 = better_first_word_with_slicing(phrase_4);

	println!("The first word in phrase_3 is '{}'", word_3);
	println!("The first word in phrase_4 is '{}'", word_4);

} //-- At this point, all variables on the stack are cleaned up. 
  //-- And all valid variables that have data on the heap are also cleaned up (dropped)

fn takes_ownership(str: String) {
	println!("takes_ownership: {}", str);
}

fn makes_copy(v: i32) {
	println!("makes_copy: {}", v);
}

fn gives_ownership() -> String {
	String::from("gives_ownership")
}

fn takes_and_gives_back(str: String) -> String {
	str
}

fn calculate_length(str: String) -> (String, usize) {
	let length = str.len();
	(str, length)
}

fn calculate_length_ref(str: &String) -> usize {  //-- str is only a reference to a String, no move 
												  //-- (therefore no ownership) has taken place. This is called borrowing - 
												  //-- as in borrowing ownership, then giving it back once the function
												  //-- goes out of scope
	str.len()
} //-- Here, str goes out of scope. But because it does not have ownership of what
  //-- it refers to, nothing happens (no cleanup/dropping)

fn change_and_calculate_length_ref(str: &mut String) -> usize {
	str.push_str(" .. I mutated your string - hah!");
	str.len()
}

fn first_word(str: &String) -> usize {
	let bytes = str.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' { 
			return i; 
		}
	}

	return str.len();
}

fn first_word_with_slicing(val: &String) -> &str {
	let bytes = val.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' { 
			return &val[..i]
		}
	}

	return &val[..]
}

fn better_first_word_with_slicing(val: &str) -> &str {
	let bytes = val.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' { 
			return &val[..i]
		}
	}

	return &val[..]
}
