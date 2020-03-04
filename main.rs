
mod question; 

fn main(){
	// let ip = vec!["cba".to_string(),"daf".into(),"ghi".into()];
	// let ip = vec!["zyx".to_string(),"wvu".into(),"tsr".into()];
	let ip = vec!["rrjk".to_string(),"furt".into(),"guzm".into()];
	println!("{:?}", question::Solution::min_deletion_size(ip));
}

