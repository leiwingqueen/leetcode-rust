/*
在由 1 x 1 方格组成的 N x N 网格 grid 中，每个 1 x 1 方块由 /、\ 或空格构成。这些字符会将方块划分为一些共边的区域。

（请注意，反斜杠字符是转义的，因此 \ 用 "\\" 表示。）。

返回区域的数目。

 

示例 1：

输入：
[
  " /",
  "/ "
]
输出：2
解释：2x2 网格如下：

示例 2：

输入：
[
  " /",
  "  "
]
输出：1
解释：2x2 网格如下：

示例 3：

输入：
[
  "\\/",
  "/\\"
]
输出：4
解释：（回想一下，因为 \ 字符是转义的，所以 "\\/" 表示 \/，而 "/\\" 表示 /\。）
2x2 网格如下：

示例 4：

输入：
[
  "/\\",
  "\\/"
]
输出：5
解释：（回想一下，因为 \ 字符是转义的，所以 "/\\" 表示 /\，而 "\\/" 表示 \/。）
2x2 网格如下：

示例 5：

输入：
[
  "//",
  "/ "
]
输出：3
解释：2x2 网格如下：

 

提示：

1 <= grid.length == grid[0].length <= 30
grid[i][j] 是 '/'、'\'、或 ' '。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/regions-cut-by-slashes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


/*
我们可以考虑把一个格子分成4块(block)，上下左右，我们分别对其做编号：
上-0，右-1，下-2，左-3。
我们尝试对其进行扁平化。
假设格子的坐标是[i,j]，block=右，则扁平化的坐标为(i*col+j)*4+1

最终我们根据每个格子的字符对其进行合并即可。
1." "。上-右，上-左，上-下 做合并
2."/"。上-左，右-下 做合并
3."\"。上-右，左-下 做合并

然后我们还需要对相邻的格子进行合并
 */
pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let size = grid.len();
    let mut uf = UnionFind::new(size * size * 4);
    for i in 0..size {
        for j in 0..size {
            let ch = grid[i].as_bytes()[j] as char;
            if ch == ' ' {
                uf.union(get_idx(i, j, 0, size), get_idx(i, j, 1, size));
                uf.union(get_idx(i, j, 0, size), get_idx(i, j, 2, size));
                uf.union(get_idx(i, j, 0, size), get_idx(i, j, 3, size));
            } else if ch == '/' {
                uf.union(get_idx(i, j, 0, size), get_idx(i, j, 3, size));
                uf.union(get_idx(i, j, 1, size), get_idx(i, j, 2, size));
            } else {
                uf.union(get_idx(i, j, 0, size), get_idx(i, j, 1, size));
                uf.union(get_idx(i, j, 2, size), get_idx(i, j, 3, size));
            }
            //上下左右4个方向的格子做合并
            if i > 0 {
                uf.union(get_idx(i, j, 0, size), get_idx(i - 1, j, 2, size));
            }
            if i + 1 < size {
                uf.union(get_idx(i, j, 2, size), get_idx(i + 1, j, 0, size));
            }
            if j > 0 {
                uf.union(get_idx(i, j, 3, size), get_idx(i, j - 1, 1, size));
            }
            if j + 1 < size {
                uf.union(get_idx(i, j, 1, size), get_idx(i, j + 1, 3, size));
            }
        }
    }
    return uf.count as i32;
}

fn get_idx(i: usize, j: usize, block: usize, col: usize) -> usize {
    return (i * col + j) * 4 + block;
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
    let grid = vec![String::from("  /"), String::from("/ ")];
    let res = regions_by_slashes(grid);
    println!("res:{}", res);
}