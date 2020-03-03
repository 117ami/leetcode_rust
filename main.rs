
mod question; 

fn main(){
	let s = "()))((";
	println!("{:?}", question::Solution::min_add_to_make_valid(s.to_string()));
}

