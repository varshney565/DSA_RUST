fn segregate(v : &mut Vec<i32>) {
    let mut i : i32 = -1;
    let mut j : i32 = 0;
    let mut k : i32 = v.len() as i32;
    while j < k {
        if v[j as usize] == 0 {
            i += 1;
            v.swap(i as usize, j as usize);
            j += 1;
        }else if v[j as usize] == 1 {
            // no need to do anything here
            j += 1;
        }else if v[j as usize] == 2 {
            k -= 1;
            v.swap(j as usize,k as usize);
        }
    }
}

fn main() {
    let mut v = vec![0,1,2,1,2,1,0,0,1,1,0,2,0,0,1,2,2,2,2,1,1,1,1,1,0,0,0,0,0,1,1,2,2,1,2];
    segregate(&mut v);
    println!("{:?}",v);
}