
mod question; 

fn main(){
	let hs = vec![1,1,1,4,2,1,3];
	println!("{:?}", question::Solution::height_checker(hs));
}

