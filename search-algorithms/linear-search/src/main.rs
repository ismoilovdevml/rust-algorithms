// linear search algorithm in Rust

fn main() {
    let mut list = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let linear = linear_search(&list, 17);
    println!("Natija: {:?}", linear);
}

pub fn linear_search(list: &[i32], target: i32) -> Option<usize> {
    for (index, &item) in list.iter().enumerate(){
        if item == target {
            return Some(index);
        }
    }
    None
}