
mod question; 

fn main(){
	let arr = vec![2,2,3,4];
	println!("{:?}", question::Solution::find_lucky(arr));
}

