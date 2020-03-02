
mod aux; 
mod question; 

fn main(){
	let words:Vec<String> = vec!["gin".to_string(), "zen".into(), "gig".into(), "msg".into()]; 
	println!("{:?}", question::Solution::unique_morse_representations(words));
	println!("{}", 'a' as u32 - 97);
}

