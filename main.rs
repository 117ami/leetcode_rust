
mod question; 

fn main(){
	let a = vec![0,1,2,3,4,5,6,7,16,8];
	println!("{:?}", question::Solution::sort_by_bits(a));
}

