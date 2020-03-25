
mod question; 

fn main(){
	let nums1 = vec![4,9,5];
	let nums2 = vec![9,4,9,8,4];
	println!("{:?}", question::Solution::intersect(nums1, nums2));
}

