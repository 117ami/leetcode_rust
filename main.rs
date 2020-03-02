
mod aux; 
mod question; 

fn main(){
	let a = vec![vec![1,1,0], vec![1,0,1], vec![0,0,0]];
	println!("{:?}", question::Solution::flip_and_invert_image(a));
}

