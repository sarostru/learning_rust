
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

struct Node {
	value: Option<String>,
	children: Vec<Box<Node>>,
}

/// Need to place the tokens into a Tree of Tokens
/// This approach is very broken, stopping for now
// fn make_tree_from_tokens(tokens: Vec<String>) -> Box<Node> {
	// let mut root = Box::new(Node { value: None, children: vec![]});
	// if tokens.is_empty() {
		// println!("Error: Unexpected EOF");
	// }
	// let mut parent = &root; 
	// let mut child = &root;
	// for token in &tokens {
		// match token {
			// "(" => {
				// child = Box::new(Node { value: None, children: vec![]}));
			// },
			// ")" => {
				// curr_node.children.push(curr);
			// },
			// _ => {
				// curr.push(Box::new(Node { value: Some(token), children: vec![]}));
			// }
		// }
	// }
	// root
// }


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
		//let tree = make_tree_from_tokens(tokens);
		
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
