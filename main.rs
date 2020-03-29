
mod question; 

fn main(){
	let mut ns = vec![1,0,3,12,0];
	println!("{:?}", question::Solution::move_zeroes(&mut ns));
}

