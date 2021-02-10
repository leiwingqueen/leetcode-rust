/*
904. 水果成篮

在一排树中，第 i 棵树产生 tree[i] 型的水果。
你可以从你选择的任何树开始，然后重复执行以下步骤：

把这棵树上的水果放进你的篮子里。如果你做不到，就停下来。
移动到当前树右侧的下一棵树。如果右边没有树，就停下来。
请注意，在选择一颗树后，你没有任何选择：你必须执行步骤 1，然后执行步骤 2，然后返回步骤 1，然后执行步骤 2，依此类推，直至停止。

你有两个篮子，每个篮子可以携带任何数量的水果，但你希望每个篮子只携带一种类型的水果。

用这个程序你能收集的水果树的最大总量是多少？

 

示例 1：

输入：[1,2,1]
输出：3
解释：我们可以收集 [1,2,1]。
示例 2：

输入：[0,1,2,2]
输出：3
解释：我们可以收集 [1,2,2]
如果我们从第一棵树开始，我们将只能收集到 [0, 1]。
示例 3：

输入：[1,2,3,2,2]
输出：4
解释：我们可以收集 [2,3,2,2]
如果我们从第一棵树开始，我们将只能收集到 [1, 2]。
示例 4：

输入：[3,3,3,1,2,1,1,2,3,3,4]
输出：5
解释：我们可以收集 [1,2,1,1,2]
如果我们从第一棵树或第八棵树开始，我们将只能收集到 4 棵水果树。
 

提示：

1 <= tree.length <= 40000
0 <= tree[i] < tree.length

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/fruit-into-baskets
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
最多包含2个不同数字的最大子串
 */

pub fn total_fruit(tree: Vec<i32>) -> i32 {
    let len = tree.len();
    let mut freq = vec![0; len];
    let mut cnt = 0;
    let (mut l, mut r) = (0usize, 0usize);
    let mut res = 0;
    while r < len {
        //窗口右移
        let r_idx = tree[r] as usize;
        freq[r_idx] += 1;
        if freq[r_idx] == 1 {
            cnt += 1;
        }
        r += 1;
        //窗口左移
        while cnt > 2 {
            let l_idx = tree[l] as usize;
            freq[l_idx] -= 1;
            if freq[l_idx] == 0 {
                cnt -= 1;
            }
            l += 1;
        }
        res = if r - l > res { r - l } else { res };
    }
    return res as i32;
}

#[test]
fn test() {
    let res = total_fruit(vec![1, 2, 1]);
    println!("{}", res);
}
