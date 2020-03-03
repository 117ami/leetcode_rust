
mod question; 

fn main(){
	let s= "leetcode"; 
	let t = "practice";
	println!("{:?}", question::Solution::min_steps(s.to_string(), t.to_string()));
}

