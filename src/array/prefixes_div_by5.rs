/*
给定由若干 0 和 1 组成的数组 A。我们定义 N_i：从 A[0] 到 A[i] 的第 i 个子数组被解释为一个二进制数（从最高有效位到最低有效位）。

返回布尔值列表 answer，只有当 N_i 可以被 5 整除时，答案 answer[i] 为 true，否则为 false。

 

示例 1：

输入：[0,1,1]
输出：[true,false,false]
解释：
输入数字为 0, 01, 011；也就是十进制中的 0, 1, 3 。只有第一个数可以被 5 整除，因此 answer[0] 为真。
示例 2：

输入：[1,1,1]
输出：[false,false,false]
示例 3：

输入：[0,1,1,1,1,1]
输出：[true,false,false,false,true,false]
示例 4：

输入：[1,1,1,0,1]
输出：[false,false,false,false,false]
 

提示：

1 <= A.length <= 30000
A[i] 为 0 或 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/binary-prefix-divisible-by-5
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

///典型的位运算题目。
/// f(i)=f(i-1)<<1+a[i]
/// 不通过，A.length最大可以达到3000，无论多少位的整形都存不下。。
pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    let mut res = vec![];
    let mut pre = 0;
    for num in a {
        let n = (pre << 1) + num;
        //println!("n:{}", n);
        res.push(n % 5 == 0);
        pre = n;
    }
    return res;
}

///
/// 我们先来一个结论f(i)%5=(f(i-1)%5*2+k)%5，因此我们每次只需要保留上一位的取模结果即可
/// 我们把f(i)%5看成一个function m(i)，则有m(i)=(m(i-1)*2+k)%5
///
/// 模运算法则
///
/// (a + b) % p = (a % p + b % p) % p
///
///
///
///
pub fn prefixes_div_by5_2(a: Vec<i32>) -> Vec<bool> {
    let mut res = vec![];
    let mut pre = 0;
    for num in a {
        let n = ((pre << 1) + num) % 5;
        //println!("n:{}", n);
        res.push(n == 0);
        pre = n;
    }
    return res;
}

#[test]
fn test() {
    let res = prefixes_div_by5(vec![0, 1, 1]);
    for x in res {
        println!("{}", x);
    }
}

#[test]
fn test2() {
    let array = vec![1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1];
    let res = prefixes_div_by5_2(array);
    for x in res {
        println!("{}", x);
    }
}

