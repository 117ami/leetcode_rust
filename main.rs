
mod question; 

fn main(){
	let arr = vec![1,2,2,1,1,3,2];
	println!("{:?}", question::Solution::unique_occurrences(arr));
}

