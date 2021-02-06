/*
几张卡牌 排成一行，每张卡牌都有一个对应的点数。点数由整数数组 cardPoints 给出。

每次行动，你可以从行的开头或者末尾拿一张卡牌，最终你必须正好拿 k 张卡牌。

你的点数就是你拿到手中的所有卡牌的点数之和。

给你一个整数数组 cardPoints 和整数 k，请你返回可以获得的最大点数。

 

示例 1：

输入：cardPoints = [1,2,3,4,5,6,1], k = 3
输出：12
解释：第一次行动，不管拿哪张牌，你的点数总是 1 。但是，先拿最右边的卡牌将会最大化你的可获得点数。最优策略是拿右边的三张牌，最终点数为 1 + 6 + 5 = 12 。
示例 2：

输入：cardPoints = [2,2,2], k = 2
输出：4
解释：无论你拿起哪两张卡牌，可获得的点数总是 4 。
示例 3：

输入：cardPoints = [9,7,7,9,7,7,9], k = 7
输出：55
解释：你必须拿起所有卡牌，可以获得的点数为所有卡牌的点数之和。
示例 4：

输入：cardPoints = [1,1000,1], k = 1
输出：1
解释：你无法拿到中间那张卡牌，所以可以获得的最大点数为 1 。
示例 5：

输入：cardPoints = [1,79,80,1,1,1,200,1], k = 3
输出：202
 

提示：

1 <= cardPoints.length <= 10^5
1 <= cardPoints[i] <= 10^4
1 <= k <= cardPoints.length

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-points-you-can-obtain-from-cards
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
这题目等价于找到长度为n-k的最小值

时间复杂度O(n)
空间复杂度O(1)
 */
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    //先统计总值
    let mut sum = 0;
    for num in card_points.iter() {
        sum += num;
    }
    let (len, k) = (card_points.len(), k as usize);
    if len == k {
        return sum;
    }
    let (mut l, mut r, size) = (0, 0, len - k);
    let mut window = 0;
    let mut res = sum;
    while r < len {
        //窗口右移
        window += card_points[r];
        r += 1;
        if r - l == size {
            res = if window < res { window } else { res };
        } else if r - l > size {
            //窗口平移
            window -= card_points[l];
            l += 1;
            res = if window < res { window } else { res };
        }
    }
    return sum - res;
}

#[test]
fn test() {
    let points = vec![1, 2, 3, 4, 5, 6, 1];
    let res = max_score(points, 3);
    println!("{}", res);
}

#[test]
fn test_2() {
    let points = vec![100, 40, 17, 9, 73, 75];
    let res = max_score(points, 3);
    println!("{}", res);
    assert_eq!(248, res);
}