
mod question; 

fn main(){
	let light = vec![2,1,4,3,6,5];
	println!("{:?}", question::Solution::num_times_all_blue(light));
}

