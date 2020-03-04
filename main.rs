
mod question; 

fn main(){
	let a = vec![4,3,9,8];
	println!("{:?}", question::Solution::sort_array_by_parity_ii(a));
}

