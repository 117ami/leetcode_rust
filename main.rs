
mod question; 

fn main(){
	// let m = vec![0,3,3];
	let m = vec![0,1,2,3,4,5,6,7,8,9,8];
	// let m = vec![9,8,7,6,5,4,3,2,1,0];
	println!("{:?}", question::Solution::valid_mountain_array(m));
}

