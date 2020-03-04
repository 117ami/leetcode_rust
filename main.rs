
mod question; 

fn main(){
	let s = "ababcbacadefegdehijhklij";
	println!("{:?}", question::Solution::partition_labels(s.to_string()));
}

