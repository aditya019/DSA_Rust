fn main() {
    let mut arr = [4,3,2,1];
    println!("{:?}", arr);
    merge_sort(&mut arr);
    println!("{:?}", arr);
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len() as i32 - 1;
    merge_sort_util(arr, 0, len);
}

fn merge_sort_util(arr: &mut [i32], left: i32, right: i32) {
    if left >= right {
        return;
    }
    let mid = (left + right) / 2;
    merge_sort_util(arr, left, mid);
    merge_sort_util(arr, mid + 1, right);
    merge(arr, left, mid, right);
}

fn merge(arr: &mut [i32], left: i32, mid: i32, right: i32) {
    let mut temp = vec![0; (right - left + 1) as usize];
    let (mut i, mut j) = (left as usize, (mid + 1) as usize);
    let mut c = 0;
    while i <= mid as usize && j <= right as usize {
        if arr[i] <= arr[j] {
            temp[c] = arr[i];
            i += 1;
        } else {
            temp[c] = arr[j];
            j += 1;
        }
        c += 1;
    }
    while i <= mid as usize {
        temp[c] = arr[i];
        i += 1;
        c += 1;
    } 
    while j <= right as usize {
        temp[c] = arr[j];
        c += 1;
        j += 1;
    }
    let mut c = 0;
    for i in left..=right {
        arr[i as usize] = temp[c];
        c += 1;
    }
}
