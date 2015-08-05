//! Quantified Boolean Formulae 
//!
//! Implementation of QBF as used as an example in Taha's paper 
//! [DSL in MetaOCaml, Haskell, and C++] (http://www.cs.rice.edu/~taha/publications/journal/dspg04b.pdf)
//!
//! Implementation appraoch heavily inspired by Peter Norvig's [Lisp in Python] (http://norvig.com/lispy.html)
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::ascii::AsciiExt;

enum Expression {
	True,
	False,
	And (Box<Expression>, Box<Expression>),
}

impl FromStr for Expression {
    type Err = ();

    fn from_str(s: &str) -> Result<Expression, ()> {
        match s {
            "true" => Ok(Expression::True),
            "false" => Ok(Expression::False),
			"true ^ false" => Ok(Expression::And(Box::new(Expression::True), Box::new(Expression::False))),
            _ => Err(()),
        }
    }
}

/// Simple Lispy tokenizing
fn tokenize(s: &str) -> Vec<String> {
	s.replace("(", " ( ")
		.replace(")", " ) ")
		.split_whitespace()
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
}


// Tried again, but its also broken
enum SExpression {
	Literal(String),
	Internal(Vec<Box<SExpression>>),
	Root,
}

fn make_tree(tokens: &Vec<String>) -> Box<SExpression> {
	let mut tokens = (*tokens).to_vec();
	tokens.reverse();
	make_tree_from_tokens(&mut tokens)
}

fn make_tree_from_tokens(tokens: &mut Vec<String>) -> Box<SExpression> {
	let mut curr = Box::new(SExpression::Root);
	let token = (*tokens).pop();
	let token = match token {
		Some(x) => x,
		None => {
			println!("Error: Unexpected end of input");
			return curr;
		},
	};
	if &token == "(" {
		println!("Matched opening '('");
		curr = Box::new(SExpression::Internal(vec![]));
		loop {
			let last_token = match tokens.last() {
				Some(x) => x.to_string(),
				None => {
					println!("Error: Syntax Error");
					break;
				},
			};
			match last_token.as_ref() {
				")" => {
					println!("Matched closing ')'");
					break;
				},
				_ => {
					match *curr {
						SExpression::Internal(ref mut v) => v.push(make_tree_from_tokens(tokens)),
						SExpression::Literal(ref s) => break,
						SExpression::Root => break,
					}
				}
			}
		}
		(*tokens).pop();
		return curr;
	} else if &token == ")" {
		println!("Error: Syntax Error");
		return curr;
	} else {
		println!("Matched '{}'", token);
		return Box::new(SExpression::Literal(token));
	}
	return curr
}


fn eval(e: Expression) -> bool {
	match e {
		Expression::True => true,
		Expression::False => false,
		Expression::And(l, r) => {
			let l = eval(*l);
			let r = eval(*r);
			print!(" {} and {}", l, r);
			let l_and_r = l && r;
			l_and_r
		}
	}
}

fn main() {
    println!("Quantified Boolean Formulae (QBF) Interpreter.");
	
	'mainloop: loop {
		print!("> ");
        io::stdout().flush().ok().expect("Failed to flush stdout");
		
		let mut input = String::new();
		io::stdin().read_line(&mut input)
			.ok()
			.expect("Failed to read line");
		
		let tokens = tokenize(&input);
		println!("Tokens:");
		for tok in &tokens {
			print!("{}, ", tok);
		}
		println!("");
		let tree = make_tree(&tokens);
		
		let e = match input.trim().to_ascii_lowercase().parse::<Expression>() {
			Ok(e) => e,
			Err(_) => {
				println!(">> Invalid Input \"{}\"", input.trim());
				continue;
			},
		};
		let r = eval(e);
		println!(">> Got {}", r);
		// match e {
			// Expression::True => println!(">> Got True"),
			// Expression::False => println!(">> Got False"),
			// Expression::And(l, r) => println!(">> Got And"),
		// }
		//println!(">> {}", input);
		
	
	}
}
