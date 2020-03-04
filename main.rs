
mod question; 

fn main(){
	let s = "DDI";
	println!("{:?}", question::Solution::di_string_match(s.to_string()));
}

