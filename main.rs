
mod question; 

fn main(){
	let a = vec![1,2,4,5];
	println!("{:?}", question::Solution::is_monotonic(a));
}

