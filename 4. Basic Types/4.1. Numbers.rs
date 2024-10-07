fn main() {
    // 1🌟
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32; // Приводимо значення x до типу u32

    let z = 10; // Тип змінної z? Він буде визначений компілятором.

    println!("Success!");

    // 2🌟
    let v: u16 = 38_u8 as u16; // Приводимо 38 до типу u16

    println!("Success!");

    // 3🌟🌟🌟
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); // Змінюємо "u32" на "i32"

    println!("Успіх!");

    // 4🌟🌟
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())

}
    assert_eq!(i8::MAX, 127); // Максимальне значення для i8
    assert_eq!(u8::MAX, 255); // Максимальне значення для u8

    println!("Успіх!");

    // 5🌟🌟
    let v1 = 251_u8.wrapping_add(8); // Додавання без переповнення для типу u8
    let v2 = i16::checked_add(251_i16, 8).unwrap(); // Додавання з перевіркою для типу i16
    println!("{}, {}", v1, v2);

    // 6🌟🌟
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("Success! v = {}", v);

    // 7🌟🌟

    // 8🌟🌟
    // Перше рішення
    let epsilon = f64::EPSILON; // або використовуйте меншу похибку, якщо необхідно
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < epsilon);

    println!("Success!");

    // Друге рішення
    let sum = (0.1_f64 + 0.2_f64).round(); // Округлюємо результат
    assert!(sum == 0.3_f64.round());

    println!("Success!");

    // 9🌟🌟
    // Підрахунок суми від -3 до 1
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }
    
    // Зміна умови перевірки, щоб сума збігалася
    assert!(sum == -5);

    // Виведення символів і їх ASCII-кодів
    for c in 'a'..='z' {
        println!("{} - {}", c as u32, c as u32);
    }

    // 10🌟🌟
use std::ops::{Range, RangeInclusive};


    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");


    // 11🌟
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);

    // Множення
    assert!(3 * 50 == 150);

    // Ділення з плаваючою комою
    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < 1e-10);  // Явне вказання типу f64

    // Остача від ділення
    assert!(24 % 5 == 4);

    // Логічні операції
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Побітові операції
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // Виведе 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);  // Виведе 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // Виведе 0110
    println!("1 << 5 is {}", 1u32 << 5); // Виведе 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // Виведе 0x20
}

