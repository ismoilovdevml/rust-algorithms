// linear search va binary search algoritmlari

// binary search tartibli ro'yxatlarda ishlaydi 

// linear searchga farqi yo'q
// linear search har bir elementni ko'rib chiqadi
// binary search log2(element soni N) = qadam soni
log2(16) = 4
fn main() {
    let mut list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut binary = binary_search(&list, 8);
    println!("Natija: {:?}",binary);

}

pub fn binary_search(list: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len() - 1;


    while left <= right {
        let mid = (left + right) / 2;
        let guess = list[mid];

        if guess == target {
            return Some(mid);
        } else if guess > target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}