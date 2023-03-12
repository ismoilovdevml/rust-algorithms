# Bubble Sort Algorithm

Bubble Sort - bu eng oddiy sorting(tartiblash) algoritmi bo'lib, agar ular noto'g'ri tartibda bo'lsa, adjacent elementlarni qayta-qayta almashtirish orqali ishlaydi. Ushbu algoritm katta ma'lumotlar to'plamlari uchun mos emas, chunki uning average va worst-case time(eng yomon vaqt murakkabligi) ancha yuqori.


![alt text](https://www.swtestacademy.com/wp-content/uploads/2021/11/bubble-sort-animation-swtestacademy-bg.gif)
![lat text](https://upload.wikimedia.org/wikipedia/commons/c/c8/Bubble-sort-example-300px.gif?20131109191607)


## Rust 

```rust
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
```