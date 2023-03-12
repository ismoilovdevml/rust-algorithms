fn main() {
    let mut arr = [4, 7, 9, 1, 2, 3, 0, 8, 5, 6, -3, 234, -23, 11];
    println!("Tartibisz ro'yxat: {:?}", arr);

    buble_sort(&mut arr);
    println!("Tartiblangan ro'yxat: {:?}", arr);
}

pub fn buble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
