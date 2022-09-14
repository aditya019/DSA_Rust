fn main() {
    let mut v = vec![5,4,3,2,1];
    println!("{:?}", v);
    quick_sort(&mut v);
    println!("{:?}", v);
}

fn quick_sort(v: &mut [i32]) {
    quick_sort_util(v, 0, v.len() as i32 - 1);
}

fn quick_sort_util(v: &mut [i32], left: i32, right: i32) {
    if left > right {
        return;
    }
    let pivot = v[right as usize];
    let (mut l, mut r) = (left, right);
    while l < r {
        while v[l as usize] <= pivot && l < r {
            l += 1;
        } 
        while v[r as usize] >= pivot && l < r {
            r -= 1;
        }
        swap(v, l, r);
    }
    swap(v, l, right);
    quick_sort_util(v, left, l - 1);
    quick_sort_util(v, l + 1, right);
    
}

fn swap(v: &mut [i32], index1: i32, index2: i32) {
    let temp = v[index1 as usize];
    v[index1 as usize] = v[index2 as usize];
    v[index2 as usize] = temp;
}
