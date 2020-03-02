
mod aux; 
mod question; 

fn main(){
	let a = vec![1,2,3,3];
	println!("{:?}", question::Solution::repeated_n_times(a));
}

