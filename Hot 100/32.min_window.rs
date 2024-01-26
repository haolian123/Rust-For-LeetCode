/*
    题目：76. 最小覆盖子串
    链接：https://leetcode.cn/problems/minimum-window-substring/
 */
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut window = HashMap::new();
        let mut need = HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }

        let mut left = 0;
        let mut right = 0;
        let mut valid = 0;
        let mut min_len = usize::MAX;
        let mut res = (0, 0);
        let s_chars: Vec<char> = s.chars().collect();

        while right < s.len() {
            let c = s_chars[right];
            right += 1;

            if need.contains_key(&c) {
                *window.entry(c).or_insert(0) += 1;
                if window[&c] == need[&c] {
                    valid += 1;
                }
            }

            while valid == need.len() {
                if right - left < min_len {
                    min_len = right - left;
                    res = (left, right);
                }

                let d = s_chars[left];
                left += 1;

                if need.contains_key(&d) {
                    if window[&d] == need[&d] {
                        valid -= 1;
                    }
                    *window.get_mut(&d).unwrap() -= 1;
                }
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            s[res.0..res.1].to_string()
        }
    }
}
