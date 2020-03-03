
mod aux; 
mod question; 

fn main(){
	let mut gs = vec![2,1,2];
	gs = vec![2,1,3,3,3,2];
	println!("{:?}", question::Solution::group_the_people(gs));
}

