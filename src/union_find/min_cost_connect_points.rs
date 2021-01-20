/*
给你一个points 数组，表示 2D 平面上的一些点，其中 points[i] = [xi, yi] 。

连接点 [xi, yi] 和点 [xj, yj] 的费用为它们之间的 曼哈顿距离 ：|xi - xj| + |yi - yj| ，其中 |val| 表示 val 的绝对值。

请你返回将所有点连接的最小总费用。只有任意两点之间 有且仅有 一条简单路径时，才认为所有点都已连接。

 

示例 1：



输入：points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
输出：20
解释：

我们可以按照上图所示连接所有点得到最小总费用，总费用为 20 。
注意到任意两个点之间只有唯一一条路径互相到达。
示例 2：

输入：points = [[3,12],[-2,5],[-4,1]]
输出：18
示例 3：

输入：points = [[0,0],[1,1],[1,0],[-1,1]]
输出：4
示例 4：

输入：points = [[-1000000,-1000000],[1000000,1000000]]
输出：4000000
示例 5：

输入：points = [[0,0]]
输出：0
 

提示：

1 <= points.length <= 1000
-106 <= xi, yi <= 106
所有点 (xi, yi) 两两不同。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/min-cost-to-connect-all-points
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
kruskal算法，最小生成树
 */
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let len = points.len();
    if len == 0 {
        return 0;
    }
    //构造边的集合
    let mut edges = vec![];
    for i in 0..len - 1 {
        for j in i + 1..len {
            let p1 = points[i][0] - points[j][0];
            let p2 = points[i][1] - points[j][1];
            //let dis = std::num::abs(points[i][0] - points[j][0]) + std::num::abs(points[i][1] - points[j][1]);
            let dis = p1.abs() + p2.abs();
            edges.push(Edge { x: i, y: j, len: dis });
        }
    }
    //排序
    edges.sort_by(|e1, e2| e1.len.cmp(&e2.len));
    //遍历边
    let mut uf = UnionFind::new(points);
    let mut res = 0;
    let mut count = 0;
    for edge in edges.iter() {
        if uf.union(edge.x, edge.y) {
            res += edge.len;
            count += 1;
            if count == len - 1 {
                break;
            }
        }
    }
    return res;
}

/*
并查集来判断连通性
 */
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(points: Vec<Vec<i32>>) -> UnionFind {
        let mut parent = vec![0; points.len()];
        for i in 0..points.len() {
            parent[i] = i;
        }
        let uf = UnionFind { parent };
        return uf;
    }

    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        return self.parent[x];
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rootx = self.find(x);
        let rooty = self.find(y);
        return if rootx == rooty {
            false
        } else {
            self.parent[rootx] = rooty;
            true
        };
    }
}

struct Edge {
    x: usize,
    y: usize,
    len: i32,
}

#[test]
fn test() {
    let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    let res = min_cost_connect_points(points);
    println!("res:{}", res);
}