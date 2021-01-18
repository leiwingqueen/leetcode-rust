/*
给定一个整数数组  nums 和一个正整数 k，找出是否有可能把这个数组分成 k 个非空子集，其总和都相等。

示例 1：

输入： nums = [4, 3, 2, 3, 5, 2, 1], k = 4
输出： True
说明： 有可能将其分成 4 个子集（5），（1,4），（2,3），（2,3）等于总和。
 

提示：

1 <= k <= len(nums) <= 16
0 < nums[i] < 10000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/partition-to-k-equal-sum-subsets
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
回溯法

不通过，超出时间限制
 */
pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    //计算每个子集应该的和
    let mut sum = 0;
    //坑，这里如果是直接写nums来迭代，后面会导致nums的所有权转移，导致后面无法正常使用nums
    for x in &nums {
        sum += x;
    }
    if sum % k != 0 { return false; }
    let mut path = vec![0; k as usize];
    return backtrack(&mut path, 0, nums.len(), &nums, sum / k, k);
}

fn backtrack(path: &mut Vec<i32>, idx: usize, len: usize, nums: &Vec<i32>, sum: i32, k: i32) -> bool {
    if idx == len {
        return true;
    }
    //尝试放入对应的每个子集中
    for i in 0..k as usize {
        if path[i] + nums[idx] <= sum {
            path[i] += nums[idx];
            let res = backtrack(path, idx + 1, len, nums, sum, k);
            if res { return true; }
            //回溯
            path[i] -= nums[idx];
        }
        //优化点1.如果当前分组为0，则没必要尝试下一个分组
        if path[i] == 0 {
            break;
        }
    }
    //所有结果都尝试了，不行就返回false
    return false;
}


#[test]
fn test() {
    let nums = vec![4, 3, 2, 3, 5, 2, 1];
    let res = can_partition_k_subsets(nums, 4);
    println!("res:{}", res);
    assert!(res);
}