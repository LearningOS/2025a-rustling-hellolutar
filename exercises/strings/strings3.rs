// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut ret = "".to_string();
    let mut left = 0;
    for ch in input.chars() {
        if ch != ' ' {
            break;
        }
        left += 1;
    }
    let bytes = input.as_bytes();
    let mut right = bytes.len() - 1;
    while right > 0 {
        if bytes[right] as char != ' ' {
            break;
        };
        right -= 1;
    }
    input[left..right + 1].to_string()
}

fn compose_me(input: &str) -> String {
    let ret = input.to_string() + " world!";
    ret
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
