
mod question; 

fn main(){
	let arr = vec![1,2,2,6,6,6,6,7,10];
	println!("{:?}", question::Solution::find_special_integer(arr));
}

