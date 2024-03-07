fn min_window(s: String, t: String) -> String {
    let mut cnt1 : Vec<i32> = Vec::new();
    let mut cnt2 : Vec<i32> = Vec::new();
    cnt1.resize(256, 0);
    cnt2.resize(256, 0);
    for ch in t.chars() {
        cnt1[ch as usize] += 1;
    }
    let mut iter1 = s.chars().peekable();
    let mut i: i32 = 0;
    let mut iter2 = s.chars().peekable();
    let mut j: i32 = 0;
    let mut p : i32 = t.len() as i32;
    let mut start = 0;
    let mut end = -1;
    while j <= s.len() as i32 {
        while i <= j && p == 0 {
            if end == -1 || j-i+1 < end-start+1 {
                start = i;
                end = j;
            }
            let ch = *iter1.peek().unwrap();
            i += 1;
            cnt2[ch as usize] -= 1;
            if cnt2[ch as usize]  < cnt1[ch as usize] {
                p += 1;
            }
            iter1.next();
        }
        if j == s.len() as i32 {
            break;
        }
        let ch = *iter2.peek().unwrap();
        cnt2[ch as usize] += 1;
        if cnt2[ch as usize] <= cnt1[ch as usize] {
            p -= 1;
        }
        j += 1;
        iter2.next();
    }
    if end == -1 {
        return String::from("");
    }
    String::from(&s[start as usize..end as usize])
}

fn main() {
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    let ans = min_window(s,t);
    println!("{:?}",ans);
}
