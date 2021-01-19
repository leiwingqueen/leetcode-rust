/*
给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。

 

示例 1：

输入：grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
输出：1
示例 2：

输入：grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
输出：3
 

提示：

m == grid.length
n == grid[i].length
1 <= m, n <= 300
grid[i][j] 的值为 '0' 或 '1'


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/number-of-islands
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


const DIR: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, -1], [0, 1]];

/*
并查集算法实现
 */
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
    let x = [[1, 0], [-1, 0], [0, -1], [0, 1]];
    let row = grid.len();
    if row == 0 { return 0; }
    let col = grid[0].len();
    let mut uf = UnionFind::new(&grid);
    for i in 0..row {
        for j in 0..col {
            for dir in DIR.iter() {
                if grid[i][j] == '1' {
                    grid[i][j] = '0';
                    let x = i as i32 + dir[0];
                    let y = j as i32 + dir[1];
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    if x >= 0 && x < row && y >= 0 && y < col && grid[x][y] == '1' {
                        uf.union(i * col + j, x * col + y);
                    }
                }
            }
        }
    }
    return uf.count;
}

/*
并查集实现
 */
struct UnionFind {
    parent: Vec<usize>,
    count: i32,
}

impl UnionFind {
    fn new(grid: &Vec<Vec<char>>) -> UnionFind {
        let row = grid.len();
        let col = grid[0].len();
        let mut union_find = UnionFind { parent: vec![0; row * col], count: (row * col) as i32 };
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' {
                    union_find.parent[i * col + j] = i * col + j;
                    union_find.count += 1;
                }
            }
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

    fn union(&mut self, x: usize, y: usize) {
        let rootx = self.find(x);
        let rooty = self.find(y);
        if rootx != rooty {
            self.parent[rootx] = rooty;
            self.count -= 1;
        }
    }
}

#[test]
fn test() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0']
    ];
    let res = num_islands(grid);
    println!("res:{}", res);
}