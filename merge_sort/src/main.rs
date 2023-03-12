// fn main() {
//     let mut arr = [1, 4, 6, -2, -3, 2, 7, 4, 0, 7, 23, 12, 67];
//     merge_sort(&mut arr);
//     println!("Tratib: {:?}", arr)
// }


// fn merge_sort(arr: &mut [i32]) {
//     if arr.len() > 1 {
//         let mid = arr.len() / 2;
//         let (mut left, mut right) = arr.split_at_mut(mid);


//         merge_sort(&mut left);
//         merge_sort(&mut right);

//         let mut i = 0;
//         let mut j = 0;
//         let mut k = 0;

//         let mut temp = vec![0; arr.len()];

//         while i < left.len() && j < right.len() {
//             if left[i] < right[j] {
//                 temp[k] = left[i];
//                 i += 1;
//             } else {
//                 temp[k] == right[j];
//                 j += 1;
//             }

//             k += 1;
//         }

//         while i < left.len(){
//             temp[k] = left[i];
//             i += 1;
//             k += 1;
//         }

//         while j < right.len() {
//             temp[k] = right[j];
//             j += 1;
//             k += 1;
//         }

//         for i in 0..arr.len() {
//             arr[i] = temp[i]
//         }
//     }
// }


fn main() {
    println!("Merge-Sort Algoritm\n");
    let mut raqamlar = [1, 3, 7, 9, 5, 4, 3, -1 ,-5, 12, 0, 8, 32];
    println!("Berilgan Raqamlar: \n{:?}", raqamlar);
    merge_sort(&mut raqamlar);
    println!("Tartiblangan: \n {:?}", raqamlar);

    println!("Merge-Sort harflar\n");
    let mut harflar = ["a", "e", "t", "y", "i", "f", "c", "w", "rew", "rt", "ur", "wer", "adfa", "wet"];
    println!("Berilgan Harflar: \n {:?}", harflar);
    merge_sort(&mut harflar);
    println!("Tartiblangan Harflar: \n {:?}", harflar);
}

fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k =0;

    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }

    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

fn merge_sort<T: Copy + Ord>(x: &mut [T]) {
    let n = x.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut x[0..m]);
    merge_sort(&mut x[m..n]);

    let mut y: Vec<T> = x.to_vec();

    merge(&x[0..m], &x[m..n], &mut y[..]);

    x.copy_from_slice(&y)
}