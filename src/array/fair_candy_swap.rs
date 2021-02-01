/*
888. 公平的糖果棒交换


爱丽丝和鲍勃有不同大小的糖果棒：A[i] 是爱丽丝拥有的第 i 根糖果棒的大小，B[j] 是鲍勃拥有的第 j 根糖果棒的大小。

因为他们是朋友，所以他们想交换一根糖果棒，这样交换后，他们都有相同的糖果总量。（一个人拥有的糖果总量是他们拥有的糖果棒大小的总和。）

返回一个整数数组 ans，其中 ans[0] 是爱丽丝必须交换的糖果棒的大小，ans[1] 是 Bob 必须交换的糖果棒的大小。

如果有多个答案，你可以返回其中任何一个。保证答案存在。

 

示例 1：

输入：A = [1,1], B = [2,2]
输出：[1,2]
示例 2：

输入：A = [1,2], B = [2,3]
输出：[1,2]
示例 3：

输入：A = [2], B = [1,3]
输出：[2,3]
示例 4：

输入：A = [1,2,5], B = [2,4]
输出：[5,4]
 

提示：

1 <= A.length <= 10000
1 <= B.length <= 10000
1 <= A[i] <= 100000
1 <= B[i] <= 100000
保证爱丽丝与鲍勃的糖果总量不同。
答案肯定存在。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/fair-candy-swap
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
解法1-暴力
时间复杂度O(mn)。m是a的长度，n是b的长度
空间复杂度O(1)
 */
use std::collections::HashSet;

pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut sum_a = 0;
    let mut sum_b = 0;
    for num in a.iter() {
        sum_a += *num;
    }
    for num in b.iter() {
        sum_b += *num;
    }
    //结果一定存在，这里先不考虑奇数场景
    let diff = (sum_a - sum_b) / 2;
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] - b[j] == diff {
                return vec![a[i], b[j]];
            }
        }
    }
    return vec![];
}

/*
空间换时间

时间复杂度O(m+n)
空间复杂度O(m+n)
 */
pub fn fair_candy_swap_2(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();
    for num in a.iter() {
        sum_a += *num;
        set_a.insert(*num);
    }
    for num in b.iter() {
        sum_b += *num;
        set_b.insert(*num);
    }
    //结果一定存在，这里先不考虑奇数场景
    let diff = (sum_b - sum_a) / 2;
    for i in 0..a.len() {
        let expect_b = a[i] + diff;
        if set_b.contains(&expect_b) {
            return vec![a[i], expect_b];
        }
    }
    return vec![];
}

/*
在上面基础上在做一点优化
 */
pub fn fair_candy_swap_3(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut set_b = HashSet::new();
    for num in a.iter() {
        sum_a += *num;
    }
    for num in b.iter() {
        sum_b += *num;
        set_b.insert(*num);
    }
    //结果一定存在，这里先不考虑奇数场景
    let diff = (sum_b - sum_a) / 2;
    for i in 0..a.len() {
        let expect_b = a[i] + diff;
        if set_b.contains(&expect_b) {
            return vec![a[i], expect_b];
        }
    }
    return vec![];
}

#[test]
fn test() {}
