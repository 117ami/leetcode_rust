
mod question; 

fn main(){
	let arr1 = vec![4,5,8]; 
	let arr2 = vec![10,9,1,8];
	println!("{:?}", question::Solution::find_the_distance_value(arr1, arr2, 2));
}

