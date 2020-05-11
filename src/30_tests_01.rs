pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {

}

#[cfg(test)]
mod tests2 {
    use super::*;
    #[test]
    fn in_works() {
        assert_eq!(4, internal_adder(2, 2));
        // assert_ne!(5, internal_adder(2, 2));
    }

    #[test]
    #[should_panic]
    fn another() {
        assert!(true == false);
    }

    #[test]
    fn greetings_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Max"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}

#[cfg(test)]
mod tests1 {
    #[test]
    fn in_works() {
    }

    #[test]
    fn check_two() {
        assert!(1 + 1 == 2);
    }

    #[test]
    #[should_panic]
    fn check_three() {
        assert!(3 == 4);
    }
}