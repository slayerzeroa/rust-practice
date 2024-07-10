// fn main() {
//     let x=3;
//     let result = another_function(x);
//     println!("The result is: {}", result);
// }

// fn another_function(x: u32) -> u32 {
//     return x * 5;
// }



// // not duplicate define the variables
// fn main() {
//     // let y = 6;
//     let x = (let y = 6);
//     println!("y is the {}", x);
// }



// // not duplicated but the new one define is okay
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is {}", y);
// }



fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{}, {}, {}", c, z, heart_eyed_cat)
}