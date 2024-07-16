

pub struct Solution;
impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums=Vec::<i32>::with_capacity(nums2.len()+nums2.len());
    nums.extend_from_slice(&nums1);
    nums.extend_from_slice(&nums2);

    nums.sort();

    match nums.len()&1 {
      0=> {
        let i=nums.len()/2;
        (nums[i]+nums[i-1]) as f64/2.
      },
      _=> nums[nums.len()/2] as _
    }
  }
}



fn main() {
  
}
