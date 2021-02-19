/*
给定一个字符串，你的任务是计算这个字符串中有多少个回文子串。

具有不同开始位置或结束位置的子串，即使是由相同的字符组成，也会被视作不同的子串。

 

示例 1：

输入："abc"
输出：3
解释：三个回文子串: "a", "b", "c"
示例 2：

输入："aaa"
输出：6
解释：6个回文子串: "a", "a", "a", "aa", "aa", "aaa"
 

提示：

输入的字符串长度不会超过 1000 。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/palindromic-substrings
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


/*
     * 我们考虑如何降低回文串的判断效率
     * <p>
     * 假设f(i,j)为当前字符是否回文串，i<=j
     * <p>
     * f(i,j)=f(i+1,j-1)&&s[i]==s[j]
     * <p>
     * 那么我们可以使用dp来降低回文串的计算效率
     * <p>
     * 需要注意的是由于dp迭代是基于左下方的点决定的，因此我们需要从下往上迭代

     好恶心,rust的各种类型约束
     *
     * @param s
     * @return
     */
pub fn count_substrings(s: String) -> i32 {
    let len = s.len();
    if len == 0 { return 0; }
    let mut res = 0;
    let mut dp = vec![vec![false; len]; len];
    let mut i: i32 = (len - 1) as i32;
    let s = s.as_bytes();
    while i >= 0 {
        for j in i..len as i32 {
            let (i, j) = (i as usize, j as usize);
            let (x, y) = (i as i32 + 1, j as i32 - 1);
            if s[i] == s[j] && (x > y || dp[x as usize][y as usize]) {
                dp[i][j] = true;
                res += 1;
            }
        }
        i -= 1;
    }
    return res;
}

#[test]
fn test() {
    let res = count_substrings(String::from("aaa"));
    println!("{}", res);
}