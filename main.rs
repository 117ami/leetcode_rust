
mod question; 

fn main(){
	let n = 4;
	println!("{:?}", question::Solution::generate_the_string(n));
	let s = std::iter::repeat("a").take(10).collect::<String>() + "b";
}

