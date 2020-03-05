
mod question; 

fn main(){
	let s = "()()";
	println!("{:?}", question::Solution::max_depth_after_split(s.to_string()));
}

