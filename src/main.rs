#[derive(Debug)]
struct Condition {
    value: i32,
    answer: String,
}

fn check_number(number: i32, condition: &Condition, output: String) -> String {
    let mut new_output = output.to_string();
    if number % condition.value == 0 {
        new_output = format!("{}{}", output, condition.answer);
    }
    return new_output;
}

fn main() {
    let fizz = Condition {
        value: 3,
        answer: "Fizz".to_string(),
    };
    let buzz = Condition {
        value: 5,
        answer: "Buzz".to_string(),
    };
    for number in 1..101 {
        let output = "".to_string();
        let output = check_number(number, &fizz, output);
        let output = check_number(number, &buzz, output);
        if output.chars().count() == 0 {
            println!("{}", &number.to_string());
        } else {
            println!("{}", output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz() {
        let fizz = Condition {
            value: 3,
            answer: "Fizz".to_string(),
        };
        assert_eq!(check_number(3, &fizz, "".to_string()), "Fizz");
    }

    #[test]
    fn test_buzz() {
        let buzz = Condition {
            value: 5,
            answer: "Buzz".to_string(),
        };
        assert_eq!(check_number(5, &buzz, "".to_string()), "Buzz");
    }

    #[test]
    fn test_fizzbuzz() {
        let fizz = Condition {
            value: 3,
            answer: "Fizz".to_string(),
        };
        let buzz = Condition {
            value: 5,
            answer: "Buzz".to_string(),
        };
        let output = "".to_string();
        let output = check_number(15, &fizz, output);
        let output = check_number(15, &buzz, output);
        assert_eq!(output, "FizzBuzz");
    }
}
