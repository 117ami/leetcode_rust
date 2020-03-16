
mod question; 

fn main(){
	// let v = vec!["abc".to_string(),"deq".into(),"mee".into(),"aqq".into(),"dkd".into(),"ccc".into()];
	let v = vec!["ef".to_string(),"fq".into(),"ao".into(),"at".into(),"lx".into()];
	println!("{:?}", question::Solution::find_and_replace_pattern(v, "ya".to_string()));
}

