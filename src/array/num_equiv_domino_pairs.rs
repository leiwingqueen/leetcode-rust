/*
1128. 等价多米诺骨牌对的数量

给你一个由一些多米诺骨牌组成的列表 dominoes。

如果其中某一张多米诺骨牌可以通过旋转 0 度或 180 度得到另一张多米诺骨牌，我们就认为这两张牌是等价的。

形式上，dominoes[i] = [a, b] 和 dominoes[j] = [c, d] 等价的前提是 a==c 且 b==d，或是 a==d 且 b==c。

在 0 <= i < j < dominoes.length 的前提下，找出满足 dominoes[i] 和 dominoes[j] 等价的骨牌对 (i, j) 的数量。

 

示例：

输入：dominoes = [[1,2],[2,1],[3,4],[5,6]]
输出：1
 

提示：

1 <= dominoes.length <= 40000
1 <= dominoes[i][j] <= 9

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/number-of-equivalent-domino-pairs
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

/*
由于骨牌的数字范围是1~9，因为4个bit就能够存储，两个数字的话8个bit就足够了。
我们初始化一个hashmap进行存储和记录即可。
 */
use std::collections::HashMap;

pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut map: HashMap<u8, i32> = HashMap::new();
    let mut res = 0;
    for dom in dominoes.iter() {
        let key = hash_key((*dom)[0], (*dom)[1]);
        //这里必须clone，会报错，map被引用为immutable,后面insert的时候又变成mutable
        let option = map.get(&key).cloned();
        match option {
            Some(v) => {
                map.insert(key, v + 1);
                res += v;
            }
            None => {
                map.insert(key, 1);
            }
        }
    }
    return res;
}

fn hash_key(i: i32, j: i32) -> u8 {
    let mut i = i;
    let mut j = j;
    //交换元素
    if i > j {
        let tmp = i;
        i = j;
        j = tmp;
    }
    return ((i << 4) + j) as u8;
}

#[test]
fn test() {
    let dom = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];
    let res = num_equiv_domino_pairs(dom);
    println!("{}", res);
}