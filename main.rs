
mod question; 

fn main(){
	let ns = vec![8,1,2,2,3];
	println!("{:?}", question::Solution::smaller_numbers_than_current(ns));
}

