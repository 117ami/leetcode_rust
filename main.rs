
mod aux; 
mod question; 

fn main(){
	let a = vec![-4,-1,0,3,10];
	println!("{:?}", question::Solution::sorted_squares(a));
}

