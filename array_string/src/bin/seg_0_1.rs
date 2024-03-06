#[allow(dead_code)]
fn partition1(v : &mut Vec<i32>) {
    // Lomuto's partition
    let mut i: i32 = -1;let mut j = 0;
    let n = v.len() as i32;
    // partition the array into two halfs 
    // (00000)(111)(11)
    while j < n {
        if v[j as usize] == 0 {
            i+=1;
            v.swap(i as usize, j as usize);
        }
        j+=1;
    }
}

fn partition2(v : &mut Vec<i32>) {
    // Hoare's partition
    let mut i: i32 = -1;
    let mut j: i32 = v.len() as i32;
    while i < j {
        loop {
            i+=1;
            if v[i as usize] == 1 {
                break;
            }
        }
        loop {
            j-=1;
            if v[j as usize] == 0 {
                break;
            }
        }
        if i < j {
            v.swap(i as usize,j as usize);
            i+=1;
            j-=1;
        }
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.resize(10,0);
    v[0] = 1;v[3] = 1;v[6] = 1;v[8] = 1;
    v[9] = 1;v[5] = 1;
    partition2(&mut v);
    println!("{:?}",v);
}