
mod aux; 
mod question; 

fn main(){
	// let s = "(()())(())";
	let mut s = "(()())(())(()(()))";
	s = "()()";
	println!("{:?}", question::Solution::remove_outer_parentheses(s.to_string()));
}

