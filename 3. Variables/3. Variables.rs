fn main() {
    // 1 🌟 
    let x: i32 = 5; // Ініціалізуємо змінну x значенням 5
    let y: i32; // Залишаємо y без ініціалізації

    assert_eq!(x, 5);
    println!("Success!");
    

    // 2 🌟 
    let mut x = 1; // Оголошуємо мутабельну змінну x зі значенням 1
    x += 2; // Змінюємо значення x на 3
    
    assert_eq!(x, 3);
    println!("Success!");
    

    // 3 🌟 
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // Змінна y більше не доступна тут!
    println!("The value of x is {}", x);
    

    // 4 🌟🌟 
    define_x(); // Викликаємо функцію define_x, щоб ініціалізувати змінну x
}
fn define_x() {
    let x = "hello";
    println!("{}, world", x); // Виводимо значення x


    // 5 🌟🌟 
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5); // Зовнішній x досі дорівнює 5

    let x = 42;
    println!("{}", x); // Виводить "42".


    // 6 🌟🌟 
    let mut x: i32 = 1;
    x = 7;
    // Тут ми використовуємо тінювання та переозначення
    let x = x; // Змінна `x` тінює попередню змінну
    // Тепер можемо змінювати `x`, оскільки вона стала неявною
    let x = x + 3;

    let y = 4;
    // Тут також використовуємо тінювання
    let y = "Я також можу бути пов'язаний з текстом!"; 

    println!("Success!");


     // 7 🌟🌟 
    // Перше рішення    
    let x = 1;
    println!("The value of x is: {}", x); // Використовуйте значення x
    
    // Друге рішення 
    let x = 1;
    let x = x + 2;
    println!("The updated value of x is: {}", x); // Використати оновлене значення

     // 8 🌟🌟 
    let (mut x, y) = (1, 2); // Оголошуємо x як mut
    x += 2; // Змінюємо значення x
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");

     // 9 🌟🌟 
    let (x, y);
    (x, ..) = (3, 4); // Деструктуризація кортежу
    [.., y] = [1, 2]; // Деструктуризація масиву
    assert_eq!([x, y], [3, 2]); // Заповнюємо пропуски

    println!("Success!");
}

     
     
     
     
     
     




