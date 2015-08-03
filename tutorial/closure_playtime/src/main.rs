//! Playing around with closures
//!
//! The [closure section] (https://doc.rust-lang.org/book/closures.html) of the rust book
//! has an odd mix of very technical information while also introducing new syntax.
//! 
//! Its pretty cool how they proivde sufficient stack manipulation tools to make returning
//! a closure not too bad.  I think maybe you take what is here and define your own closure
//! factories so you don't have to do this everytime.

// Simpler case for returning a vector box
fn factory() -> Box<Vec<i32>> {
	let mut vec = Box::new(vec![1, 2, 3]);
	vec.push(4);
	vec
}

// Simpler case for returning a vector not in a box
fn vec_factory() -> Vec<i32> {
	let mut vec = vec![1, 2, 3];
	vec.push(4);
	vec
}

fn main() {
    //let f = factory_fail_1();
	//let f = factory_fail_2();
	//let f = factory_fail_3();
	//let f = factory_fail_4();
	//let f = factory_fail_5();
	let f = factory_fail_6();
	
	let answer = f(4);
	assert_eq!(vec![1, 2, 3, 4], answer);

	let f = wrong_factory_works();
	let answer = f(4);
	assert_eq!(answer, 9);
	
	let answer = factory();
	assert_eq!(vec![1, 2, 3, 4], *answer);
	
	let answer = vec_factory();
	assert_eq!(vec![1, 2, 3, 4], answer);
}

// Error message for this one isn't too clear
// Fn is a trait not a type so rust doesn't know the size
// fn factory_fail_1() -> (Fn(i32) -> Vec<i32>) {
    // let vec = vec![1, 2, 3];

    // |n| vec.push(n)
// }

// Try with a reference, but it still doesn't work, lifetime is unspecified
// fn factory_fail_2() -> &(Fn(i32) -> Vec<i32>) {
    // let vec = vec![1, 2, 3];

    // |n| vec.push(n)
// }

// Can't just make it static, our closure isn't of that type
// fn factory_fail_3() -> &'static (Fn(i32) -> Vec<i32>) {
    // let vec = vec![1, 2, 3];

    // |n| vec.push(n)
// }

// So close this time, but vec is in the factory scope not the lambda's scope
// fn factory_fail_4() -> Box<Fn(i32) -> Vec<i32>> {
    // let vec = vec![1, 2, 3];

    // Box::new(|n| vec.push(n))
// }

// This seems to be the more complex case, still doesn't work
// fn factory_fail_5() -> Box<Fn(i32) -> Vec<i32>> {
    // let mut vec = vec![1, 2, 3];
    // Box::new( move |n| {vec.push(n); vec})
// }

// IN the docs, they switch to number, lets look at it somemore
fn wrong_factory_works() -> Box<Fn(i32) -> i32> {
	let num = 5;

    Box::new(move |x| x + num)
}
// It seems like its a pretty fundamental thing, https://doc.rust-lang.org/std/cell/
// We can't use their vector example easily since we can't guarantee that different
// threads will modify the vector, so either we use the shared mutable state cell pointers
// or I think closer to the spirit of this example we use the to_owned call
fn factory_fail_6() -> Box<Fn(i32) -> Vec<i32>> {
    let vec = vec![1, 2, 3];
    Box::new(move |n| {let mut vec = vec.to_vec(); vec.push(n); vec})
}