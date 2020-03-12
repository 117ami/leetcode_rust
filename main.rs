mod question;

fn main() {
    let s = "leetcodeisgreat";
    println!(
        "{:?}",
        question::Solution::find_the_longest_substring(s.to_string())
    );
}
