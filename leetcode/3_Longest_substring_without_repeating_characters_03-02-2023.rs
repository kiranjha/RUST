// Given a string s, find the length of the longest 
// substring
//  without repeating characters.

 

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 

// Constraints:

// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

//Solution
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let n = s.len();
        let s = s.as_bytes();
        let mut set = HashMap::new();
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        while i < n && j < n {
            if !set.contains_key(&s[j]) {
                set.insert(s[j], j+1);
                j += 1;
                ans = ans.max(j - i);
            } else {
                set.remove(&s[i]);
                i += 1;
            }
        }
        ans as i32
        }
}