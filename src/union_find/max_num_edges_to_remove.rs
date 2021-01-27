/*
Alice 和 Bob 共有一个无向图，其中包含 n 个节点和 3  种类型的边：

类型 1：只能由 Alice 遍历。
类型 2：只能由 Bob 遍历。
类型 3：Alice 和 Bob 都可以遍历。
给你一个数组 edges ，其中 edges[i] = [typei, ui, vi] 表示节点 ui 和 vi 之间存在类型为 typei 的双向边。请你在保证图仍能够被 Alice和 Bob 完全遍历的前提下，找出可以删除的最大边数。如果从任何节点开始，Alice 和 Bob 都可以到达所有其他节点，则认为图是可以完全遍历的。

返回可以删除的最大边数，如果 Alice 和 Bob 无法完全遍历图，则返回 -1 。

 

示例 1：



输入：n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
输出：2
解释：如果删除 [1,1,2] 和 [1,1,3] 这两条边，Alice 和 Bob 仍然可以完全遍历这个图。再删除任何其他的边都无法保证图可以完全遍历。所以可以删除的最大边数是 2 。
示例 2：



输入：n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
输出：0
解释：注意，删除任何一条边都会使 Alice 和 Bob 无法完全遍历这个图。
示例 3：



输入：n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
输出：-1
解释：在当前图中，Alice 无法从其他节点到达节点 4 。类似地，Bob 也不能达到节点 1 。因此，图无法完全遍历。
 

提示：

1 <= n <= 10^5
1 <= edges.length <= min(10^5, 3 * n * (n-1) / 2)
edges[i].length == 3
1 <= edges[i][0] <= 3
1 <= edges[i][1] < edges[i][2] <= n
所有元组 (typei, ui, vi) 互不相同

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
优先，我们可以想象，删除类型3的一条边=删除了1条类型1和类型2的边。如果我们要考虑能够删除最多的边，我们可以考虑先删除类型1和类型2的边(相当于尽量保留类型3的边)

我们可以分别维护一个Alice和Bob的并查集
 */
pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = n as usize;
    let mut alice = UnionFind::new(n);
    let mut bob = UnionFind::new(n);
    //先遍历类型3的边
    for edge in edges.iter() {
        if edge[0] == 3 {
            let p1 = (edge[1] - 1) as usize;
            let p2 = (edge[2] - 1) as usize;
            if !alice.union(p1, p2) {
                res += 1;
            }
            bob.union(p1, p2);
        }
    }
    //分别遍历类型1,2的边
    for edge in edges.iter() {
        let p1 = (edge[1] - 1) as usize;
        let p2 = (edge[2] - 1) as usize;
        if edge[0] == 1 {
            if !alice.union(p1, p2) {
                res += 1;
            }
        } else if edge[0] == 2 {
            if !bob.union(p1, p2) {
                res += 1;
            }
        }
    }
    return if alice.count == 1 && bob.count == 1 { res } else { -1 };
}

/*
并查集实现
 */
struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let mut union_find = UnionFind { parent: vec![0; size], count: size };
        for i in 0..size {
            union_find.parent[i] = i;
        }
        return union_find;
    }

    fn find(&mut self, x: usize) -> usize {
        //声明一个新的x变量
        let mut x = x;
        while self.parent[x] != x {
            //路径压缩
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        return self.parent[x];
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rootx = self.find(x);
        let rooty = self.find(y);
        return if rootx != rooty {
            self.parent[rootx] = rooty;
            self.count -= 1;
            true
        } else {
            false
        };
    }
}

#[test]
fn test() {
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4]
    ];
    let res = max_num_edges_to_remove(4, edges);
    println!("{}", res);
}