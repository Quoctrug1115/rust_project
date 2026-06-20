// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
// Example 1:
// Input: s = "anagram", t = "nagaram"
// Output: true

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        s_chars.sort();
        t_chars.sort();
        if s_chars == t_chars {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    let result = Solution::is_anagram(s, t);
    println!("Is anagram: {}", result);
}