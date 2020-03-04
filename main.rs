
mod question; 

fn main(){
	let mt = vec![0,1,0];
	println!("{:?}", question::Solution::peak_index_in_mountain_array(mt));
}

