//! This module uses various iterators
//!
//! Samples derived from the [iterator section] (https://doc.rust-lang.org/book/iterators.html)

// I'm using the stable build, so apparently I can't use step_by
// You would have to use the nightly release
//#![feature(step_by)]

fn main() {
    println!("#Simple Range For Loop");
	for x in 0..10 {
		print!("{}, ", x);
	}
	println!("");
	println!("#Simple Range For Loop Impl");
	let mut range = 0..10;
	
	loop {
		match range.next() {
			Some(x) => {
				print!("{}, ", x);
			},
			None => { 
				println!("");
				break; 
			}
		}
	}
	
	println!("#Bad iteration over a vector");
	let nums = vec![1, 2, 3];

	for i in 0..nums.len() {
		print!("{}, ", nums[i]);
	}
	println!("");
	
	println!("#Good Iteration over a vector");
	let nums = vec![1, 2, 3];

	for num in &nums {
		print!("{}, ", num);
	}
	println!("");
	
	println!("#Good Iteration over a vector with deref");
	let nums = vec![1, 2, 3];

	for num in &nums {
		print!("{}, ", *num);
	}
	println!("");
	println!("#Good Iterator over a vector with iter()");
	let nums = vec![1, 2, 3];

	for num in nums.iter() {
		print!("{}, ", num);
	}
	println!("");
	println!("#Collect Consumer");
	// let one_to_ten = (1..11).collect::<Vec<i32>>();
	// Can leave i32 unspecified
	let one_to_ten = (1..11).collect::<Vec<_>>();
	for num in &one_to_ten {
		print!("{}, ", *num);
	}
	println!("");
	
	println!("#Find Consumer None");
	let greater_than_four = (0..4).find(|x| *x > 4);
	match greater_than_four {
	    Some(_) => println!("We got some numbers!"),
		None => println!("No numbers found :("),
	}
	println!("#Find Consumer Some");
	let greater_than_four = (0..10).find(|x| *x > 4);
	match greater_than_four {
	    Some(_) => println!("We got some numbers!"),
		None => println!("No numbers found :("),
	}

	println!("#Fold Consumer");
	let sum = (1..4).fold(0, |sum, x| sum + x);
	println!("Sum of 1..4 = {}", sum);
	
	println!("#Map Iterator Adaptor, lazy + no consumer = Nothing Happens");
	// This will compile with a warning but will not do anything
	// (1..100).map(|x| x + 1);
	
	println!("#Map iterator adaptor, wrap in a for loop for side effects");
	for _ in (1..10).map(|x| print!("{}, ", x)) {};
	println!("");
	
	println!("#Take 5 from an infinite iterator");
	//Can't use this as step_by is in the unstable branch
	//for i in (1..).step_by(5).take(5) {
	//	print!("{}, ", i);
	//}
	for i in (1..).take(5) {
		print!("{}, ", i);
	}
	println!("");
	println!("#Filter over an infinite iterator");
	// Less satisfying implementation with filter and map
	for i in (0..).filter(|&x| x % 5 == 0).map(|x| x + 1).take(5) {
		print!("{}, ", i);
	}
	println!("");
	println!("#More Filtering");
	let some_numbers = (1..)
		.filter(|&x| x % 2 == 0)
		.filter(|&x| x % 3 == 0)
		.take(5)
		.collect::<Vec<_>>();
	for num in &some_numbers {
		print!("{}, ", num);
	}
	println!("");
}
