use std::ops::Add;

struct Solution {}

/*
给定一个二维的矩阵，包含 'X' 和 'O'（字母 O）。

找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X' 填充。

示例:

X X X X
X O O X
X X O X
X O X X
运行你的函数后，矩阵变为：

X X X X
X X X X
X X X X
X O X X
解释:

被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。



来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/surrounded-regions
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
如果我们把代表元都设置为图的边缘的点，那么我们只需要判断其代表源是否在边缘便可以控制是否需要置0
 */
//const DIR: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, -1], [0, 1]];
//理论上遍历两个方向就可以了
const DIR: [[i32; 2]; 2] = [[1, 0], [0, 1]];

pub fn solve(board: &mut Vec<Vec<char>>) {
    let row = board.len();
    if row == 0 {
        return;
    }
    let col = board[0].len();
    //先做一次合并
    let mut uf = UnionFind::new(board);
    for i in 0..row {
        for j in 0..col {
            if board[i][j] == 'X' {
                continue;
            }
            for dir in DIR.iter() {
                let x = i as i32 + dir[0];
                let y = j as i32 + dir[1];
                if x < 0 || y < 0 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if x >= 0 && x < row && y >= 0 && y < col && board[x][y] == 'O' {
                    println!("合并[{},{}]和[{},{}]", i, j, x, y);
                    uf.union((i, j), (x, y));
                }
            }
        }
    }
    //然后再做一次遍历判断结点是否需要修改为X
    for i in 0..row {
        for j in 0..col {
            if board[i][j] == 'O' {
                let (x, y) = uf.find((i, j));
                print!("[{},{}]的代表元:[{},{}]\n", i, j, x, y);
                if x > 0 && x < row - 1 && y > 0 && y < col - 1 {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
    row: usize,
    col: usize,
}

impl UnionFind {
    fn new(board: &Vec<Vec<char>>) -> UnionFind {
        let row = board.len();
        let col = board[0].len();
        let mut parent = vec![0; row * col];
        for i in 0..row {
            for j in 0..col {
                parent[i * col + j] = i * col + j;
            }
        }
        let uf = UnionFind { parent, row, col };
        return uf;
    }

    fn find(&mut self, point: (usize, usize)) -> (usize, usize) {
        let mut x = self.col * point.0 + point.1;
        while self.parent[x] != x {
            //路径压缩
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x = self.parent[x];
        return (x / self.col, x % self.col);
    }

    fn union(&mut self, p1: (usize, usize), p2: (usize, usize)) -> bool {
        let root1 = self.find(p1);
        let root2 = self.find(p2);
        return if root1 == root2 {
            false
        } else {
            //选择边缘节点作为代表元
            if root1.0 == 0 || root1.0 == self.row - 1 || root1.1 == 0 || root1.1 == self.col - 1 {
                let idx = self.get_idx(&root2);
                self.parent[idx] = self.get_idx(&root1);
            } else {
                let idx = self.get_idx(&root1);
                self.parent[idx] = self.get_idx(&root2);
            }
            true
        };
    }

    fn get_idx(&self, point: &(usize, usize)) -> usize {
        point.0 * self.col + point.1
    }
}

#[test]
fn test() {
    let mut boards = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'X', 'O', 'X']];
    solve(&mut boards);
    for row in boards.iter() {
        let mut s = String::new();
        for ch in row.iter() {
            s.push_str(ch.to_string().add(",").as_str());
        }
        println!("{}", s);
    }
}

#[test]
fn test3() {
    let mut boards = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
    ];
    solve(&mut boards);
    for row in boards.iter() {
        let mut s = String::new();
        for ch in row.iter() {
            s.push_str(ch.to_string().add(",").as_str());
        }
        println!("{}", s);
    }
}

#[test]
fn test2() {
    let mut board = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'O'],
        vec!['X', 'O', 'O', 'X', 'X', 'X', 'O', 'O', 'O', 'X'],
        vec!['O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'X'],
        vec!['O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'X', 'X', 'O', 'X', 'X', 'O', 'O', 'O'],
        vec!['X', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'X', 'O'],
        vec!['X', 'O', 'X', 'O', 'O', 'X', 'X', 'O', 'X', 'O'],
        vec!['X', 'X', 'O', 'X', 'X', 'O', 'X', 'O', 'O', 'X'],
        vec!['O', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'O'],
        vec!['X', 'X', 'O', 'X', 'X', 'X', 'X', 'O', 'O', 'O']
    ];
    solve(&mut board);
    for row in board.iter() {
        let mut s = String::new();
        for ch in row.iter() {
            s.push_str(ch.to_string().add(",").as_str());
        }
        println!("{}", s);
    }
}