
fn main () {
    println!("Quick-Sort Algoritmi");
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