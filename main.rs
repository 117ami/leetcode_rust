
mod question; 

fn main(){
	let cards = vec![17,13,11,2,3,5,7];
	println!("{:?}", question::Solution::deck_revealed_increasing(cards));
}

