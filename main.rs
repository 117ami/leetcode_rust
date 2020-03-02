
mod aux; 
mod question; 

fn main(){
	let a = vec![1,2,3,4];
	println!("{:?}", question::Solution::sort_array_by_parity(a));
}

