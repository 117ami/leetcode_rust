
mod question; 

fn main(){
	// let mat = vec![vec![1,1,0,0], vec![1,0,0,0], vec![1,1,1,0]];
	let mat = vec![vec![1,1,0,0,0],vec![1,1,1,1,0],vec![1,0,0,0,0],vec![1,1,0,0,0],vec![1,1,1,1,1]];
	println!("{:?}", question::Solution::k_weakest_rows(mat, 3));
}

