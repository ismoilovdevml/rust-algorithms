// use std::array;

// fn main() {
//     let mut array = [1, 4, 6, 2, 3, 8, 0, 5];

//     selection_sort(&mut array);
//     println!("Natija: {:?}",array);
    
// }

// pub fn selection_sort(array: &mut [i32]) {
//     for i in 0..array.len() {
//         let mut min_idx = i;
//         for j in (i + 1)..array.len() {
//             if array[j] < array[min_idx] {
//                 min_idx = j;
//             }
//         }
//         array.swap(i, min_idx);
//     }
// }


fn main() {
    let mut array = [2, 1, 5, 8, 9, 3, 4, 0];
    selection_sort(&mut array);
    println!("Natija: {:?}", array);

}


pub fn selection_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        let mut min_idx = i;
        for j in (i + 1)..array.len() {
            if array[j] < array[min_idx]{
                min_idx = j;
            }
        }

        array.swap(i, min_idx);
    }
}
