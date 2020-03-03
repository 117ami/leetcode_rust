
mod question; 

fn main(){
	println!("{:?}", question::Solution::hamming_distance(1, 4));
	println!("{:?}", (0 ^ 7 as i32).count_ones() as i32);
}

