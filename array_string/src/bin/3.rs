struct Solution {

}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n : i32 = s.len() as i32;
        let mut ans : i32 = 0;
        let mut i : i32 = 0;
        let mut j : i32 = 0;
        let mut v : Vec<i32> = Vec::new();
        v.resize(256,-1);
        let mut iter = s.chars();
        while j < n {
            let ch = iter.next().unwrap();
            if v[ch as usize] >= 0 {
                i = std::cmp::max(v[ch as usize] + 1,i);
            }
            v[ch as usize] = j;
            ans = std::cmp::max(ans,j-i+1);
            j += 1;
        }
        ans
    }
}

fn main() {
    let s = String::from("abcabcbb");
    let ans = Solution::length_of_longest_substring(s);
    println!("{}",ans);
}