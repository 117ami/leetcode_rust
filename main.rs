
mod question; 

fn main(){
	let nums = vec![0,1,2,3,4]; 
	let index = vec![0,1,2,2,1];
	println!("{:?}", question::Solution::create_target_array(nums, index));
}

