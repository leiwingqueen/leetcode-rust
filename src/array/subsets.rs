//给你一个整数数组 nums ，返回该数组所有可能的子集（幂集）。解集不能包含重复的子集。
//
//  
// 示例 1：
//
// 输入：nums = [1,2,3]
// 输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
// 示例 2：
//
// 输入：nums = [0]
// 输出：[[],[0]]
//  
//
// 提示：
//
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
//
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/subsets
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


use std::ops::Add;

///递归解法。
/// 时间复杂度O(n!)，空间复杂度O(1)，
/// 不通过，这个思路是有问题的
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut visit = vec![0; nums.len()];
    return dfs(&nums, &mut visit, nums.len());
}

fn dfs(nums: &Vec<i32>, visit: &mut Vec<i32>, n: usize) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    //空集合也算一个解
    res.push(vec![]);
    for i in 0..n {
        if visit[i] == 0 {
            visit[i] = 1;
            let sub_res = dfs(nums, visit, n);
            for sub in sub_res {
                let mut v: Vec<i32> = vec!(nums[i]);
                //这里需要全部push进去
                for x in sub {
                    v.push(x);
                }
                res.push(v);
            }
            //回溯
            visit[i] = 0;
        }
    }
    return res;
}

#[test]
fn test() {
    let res = subsets(vec![1, 2, 3]);
    for row in res {
        let mut s = String::new();
        for num in row {
            s.push_str(num.to_string().add(",").as_str());
            println!("[{}]", s);
        }
    }
}