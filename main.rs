
mod question; 

fn main(){
	let mut s = vec!['a', 'b', 'c'];
	s = vec![];
	question::Solution::reverse_string(&mut s);
	println!("{:?}", s);
}

