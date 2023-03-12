<h1 align="center" style="color:orange;"><b>Sotring Algorithms - Tartiblash Algoritmlari</b></h1>

Ushbu bo'limda sizlar bilan tartiblash algoritmlarini barcha turlarini ko'rib chiqamiz. Ularni qay o'rinda, qanday ishlatishni va qanday qilib Rust dasturlash tilida kodga o'girishni misollar va qiziqarli yo'llar bilan ko'rib chiqamiz. 

**Tartibalsh Algoritmlar bu quyidagilar:**

1. Bubble sort
2. Selection sort
3. Insertion sort
4. Merge sort
5. Quick sort
6. Heap sort
7. Radix sort
8. Shell sort
9. Couting sort
10. Bucket sort

Ammo biz bularni barchasini o'rganmasakda sizlar bilan eng keraklilarini ko'rib ketamiz.

<h2 align="center"><b>Bubble Sort</b></h2>
<hr>

Bubble Sort - bu eng oddiy sorting(tartiblash) algoritmi bo'lib, agar ular noto'g'ri tartibda bo'lsa, adjacent elementlarni qayta-qayta almashtirish orqali ishlaydi. Ushbu algoritm katta ma'lumotlar to'plamlari uchun mos emas, chunki uning average va worst-case time(eng yomon vaqt murakkabligi) ancha yuqori. 

Algoritmni qanday ishlashini pastdagi misol orqali ko'rishingiz mumkin:

![alt text](https://www.swtestacademy.com/wp-content/uploads/2021/11/bubble-sort-animation-swtestacademy-bg.gif)

![alt text](https://upload.wikimedia.org/wikipedia/commons/c/c8/Bubble-sort-example-300px.gif?20131109191607)

Ko'rib turibsizki uning ishlashi juda sodda. Keling endi uni Rustda qanday qilib kod holatiga o'tkazishni ko'rib chiqamiz.

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

<br>

<h2 align="center"><b>Selection Sort</b></h2>
<hr>

Computer scienceda selection sort joyida taqqoslash algoritmi hisoblanadi. U O(n2) time complexity(vaqt murakkabligi)ga ega, bu esa uni katta roʻyxatlarda samarasiz qiladi va odatda similar insertion sortdan koʻra yomonroq ishlaydi. Selection sort o'zining soddaligi bilan ajralib turadi va muayyan vaziyatlarda, ayniqsa yordamchi xotira cheklangan bo'lsa, murakkabroq algoritmlarga nisbatan ishlash afzalliklariga ega.

![alt text](https://www.swtestacademy.com/wp-content/uploads/2021/11/selection-sort-amination.gif)

```rust
// Bu yerda rust implementation bo'ladi!
```

<h2 align="center"><b>Insertion Sort</b></h2>
<hr>

...


<h2 align="center"><b>Merge Sort</b></h2>
<hr>

...

<h2 align="center"><b>Quick Sort</b></h2>
<hr>

...