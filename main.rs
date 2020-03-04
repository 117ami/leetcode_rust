
mod question; 

fn main(){
	let n = 4; 
	let lc = vec![1,-1,3,-1];
	let rc = vec![2,-1,-1,-1];
	println!("{:?}", question::Solution::validate_binary_tree_nodes(n, lc, rc)); 
}

