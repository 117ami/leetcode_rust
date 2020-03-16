
mod question; 

fn main(){
	let mt = vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]];
	println!("{:?}", question::Solution::lucky_numbers(mt));
}

