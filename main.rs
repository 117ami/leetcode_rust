
mod question; 

fn main(){
	let s = vec!["eat".to_string(), "tea".into(), "tan".into(), "ate".into(), "nat".into(), "bat".into()];
	println!("{:?}", question::Solution::group_anagrams(s));
}

