// use std::io;

// fn main() {
//     println!("Raqam Kiriting: ");
//     let mut a = String::new();
//     io::stdin().read_line(&mut a).unwrap();
//     let a = a.trim().parse::<i32>().unwrap();

//     let natija = factorial(a);

//     println!("{} faktoriali {} ga teng", a, natija);
// }

// pub fn factorial(n: i32) -> i32 {
//     if n == 0 {
//         1
//     } else {
//         n * factorial(n -1)
//     }
// }


use std::io;

fn main() {
    println!("Raqam Kiriting:");
    let mut raqam = String::new();
    io::stdin().read_line(&mut raqam).unwrap();
    let raqam = raqam.trim().parse::<i32>().unwrap();

    let natija = faktorial(raqam);

    println!("{} faktoriali {} ga teng", raqam, natija);

    
}

pub fn faktorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * faktorial(n -1)
    }
}