impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0, nums.len() as i32 - 1);
        while low <= high {
            let mid = low + (high - low) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        -1
    }
}