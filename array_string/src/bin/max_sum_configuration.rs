use std::cmp::max;
fn max_sum(v : &Vec<i32>) -> i32 {
    let n = v.len() as i32;
    let mut sum = 0;
    let mut val = 0;
    #[allow(unused_assignments)]
    let mut ans = 0;
    for i in 0..n {
        sum += v[i as usize];
        val += i*v[i as usize];
    }
    ans = val;
    for i in 1..n {
        val = val - sum + v[i as usize - 1] * n;
        ans = max(val, ans);
    }
    ans
}

fn main() {
    let v = vec![8,3,1,2];
    let ans = max_sum(&v);
    println!("{:?}",ans);
}