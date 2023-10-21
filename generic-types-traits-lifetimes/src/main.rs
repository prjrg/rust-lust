use std::cmp::PartialOrd;

fn main() {
    // Removing Duplication By Extracting a Function
    let number_list = vec![35, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largets number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 53, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 32, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_list));

    // Generic Data Types
    // In Function Definitions
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y:4.0};

    let both_integer = Point0 {x:5, y:10};
    let both_float = Point0 {x: 1.0, y: 4.0};
    let integer_and_float = Point0{x:5, y: 4.0};

    // In Enum Definitions
}

struct Point0<T, U> {
    x: T,
    y: U,
}

struct Point<T> {
    x: T,
    y: T,
}

fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if largest > item {
            largest = item;
        }
    }
    largest
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


