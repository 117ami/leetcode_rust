
mod question; 

fn main(){
	let n = 2; let m = 3; 
	let ids = vec![vec![0, 1], vec![1, 1]];
	println!("{:?}", question::Solution::odd_cells(n, m, ids));
}

