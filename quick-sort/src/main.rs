// use rand::{thread_rng, Rng};


// pub fn quick_sort<T: Ord>(v: &mut [T]) {
//     let mut rng = thread_rng();
//     _quick_sort(v, &mut rng);
// }

// fn _quick_sort<T: Ord, R: Rng>(v: &mut [T], rng: &mut [R]) {
//     let n = v.len();

//     if n <= 1 {
//         return;
//     }

//     let pivot_idx = rng.gen_range(0..n);

//     v.swap(pivot_idx, n - 1);

//     let pivot_v = &v[n -1 ] as *const T;

//     let mut l = 0;
//     let r = n - 2;
//     loop {
//         loop{
//             if l == n -1 || &v[l] > unsafe { &*pivot_v } {
//                 break;
//             }

//             l += 1;
//         }

//         loop {
//             if r == 0 || &v[r] < unsafe { &*pivot_v } {
//                 break;
//             }

//             r -= 1;
//         }

//         if l < r {
//             v.swap(l, r)
//         } else {
//             v.swap(l, n - 1);
//             break;
//         }

//         let (left, right) = v.split_at_mut(l);
//         _quick_sort(left, rng);
//         if l != n - 1 {
//             _quick_sort(&mut right[1..], rng)
//         }
//     }
// }


// use rand::{thread_rng, Rng};

fn main () {
    println!("Quick-Sort algoritmi");
    let mut raqamlar = [4, 6, 1, 3, 7, 9, 0, 5, 34, 12, 21, 431, 42, 56, 61];
    println!("Ro'yxat:\n {:?}", raqamlar);

    quick_sort(&mut raqamlar);
    println!("Tartiblangan: \n {:?}", raqamlar);

}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort<T: Ord>(arr: &mut [T], kichik: isize, katta: isize) {
    if kichik < katta {
        let p = qism(arr, kichik, katta);
        _quick_sort(arr, kichik, p - 1);
        _quick_sort(arr, p + 1, katta);
    }
}

fn qism<T: Ord>(arr: &mut [T], kichik: isize, katta: isize) -> isize {
    let mut _pivot = katta as usize;
    let mut store_index = kichik - 1;
    let mut oxirgi_index = katta;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[_pivot] {
            store_index += 1;
        }

        oxirgi_index -= 1;

        while oxirgi_index >= 0 && arr[oxirgi_index as usize] > arr[_pivot]{
            oxirgi_index -= 1;
        }

        if store_index >= oxirgi_index {
            break;
        } else {
            arr.swap(store_index as usize, oxirgi_index as usize);
        }
    }

    arr.swap(store_index as usize, _pivot as usize);
    store_index
}