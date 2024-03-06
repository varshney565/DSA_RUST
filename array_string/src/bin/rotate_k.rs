/*
    9 7 2 8 6 3
    k = 2
    2 8 6 3 9 7
 */

fn reverse(v : &mut Vec<i32>,mut i : i32,mut j : i32) {
    while i < j {
        v.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
}

fn rotate(v : &mut Vec<i32>,mut k : i32) {
    let n: i32 = v.len() as i32;
    k = (k + n as i32)%n;

    //now rotate k
    reverse(v, 0, k-1);
    reverse(v, k, n-1);
    reverse(v, 0, n-1);
}

fn main() {
    let mut v = vec![9,7,2,5,6,7,4,3,2];
    rotate(&mut v, -2);
    println!("{:?}",v);
}