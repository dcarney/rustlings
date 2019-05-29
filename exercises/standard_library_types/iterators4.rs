// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    match num {
        1 | 2 => num,
        _ => num * factorial(num - 1),
    }
}

pub fn factorial_no_recursion(num: u64) -> u64 {
    match num {
        1 | 2 => num,
        _ => {
            // - create a range consisting of all the elements that need multiplying
            // - example: 6 would produce the range [2..7] == [2, 3, 4, 5, 6]
            // - call fold, which takes a starting value for an "accumulator", and a closure that
            //   takes the accumulated value, and the next value in the iterator to accumulate
            (2..num + 1).fold(1, |acc, v| acc * v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
        assert_eq!(1, factorial_no_recursion(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
        assert_eq!(2, factorial_no_recursion(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
        assert_eq!(24, factorial_no_recursion(4));
    }
}

// In an imperative language you might write a for loop to iterate through
// multiply the values into a mutable variable. Or you might write code more
// functionally with recursion and a match clause. But you can also use ranges
// and iterators to solve this in rust.
