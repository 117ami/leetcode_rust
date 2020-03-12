mod question;

fn main() {
    let s = "aaaabbbbcccc";
    println!("{:?}", question::Solution::sort_string(s.to_string()));
    // println!("{:?}", 'a' as usize);
    // println!("{:?}", 97 as char);
}
