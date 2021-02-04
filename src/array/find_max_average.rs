/*
643. 子数组最大平均数 I


给定 n 个整数，找出平均数最大且长度为 k 的连续子数组，并输出该最大平均数。

 

示例：

输入：[1,12,-5,-6,50,3], k = 4
输出：12.75
解释：最大平均数 (12-5-6+50)/4 = 51/4 = 12.75
 

提示：

1 <= k <= n <= 30,000。
所给数据范围 [-10,000，10,000]。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-average-subarray-i
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
滑动窗口
 */
use std::i32::MIN;

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;
    let mut max = MIN;
    //滑动窗口
    let mut l = 0;
    let mut r = 0;
    let k = k as usize;
    while r <= nums.len() {
        if r - l < k {
            sum += nums[r];
            r += 1;
        } else {
            max = if sum > max { sum } else { max };
            if r < nums.len() {
                //更新窗口
                sum += nums[r];
                sum -= nums[l];
            }
            l += 1;
            r += 1;
        }
    }
    return max as f64 / k as f64;
}

#[test]
fn test() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let res = find_max_average(nums, 4);
    println!("{}", res);

    let nums = vec![5];
    let res = find_max_average(nums, 1);
    println!("{}", res);
}