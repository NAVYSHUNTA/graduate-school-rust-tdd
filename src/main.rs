fn main() {
}

// ここから下が本題
#[derive(PartialEq)]
struct Dollar {
    amount: i32,
}

fn times(multiplier: i32, dollar: &Dollar) -> Dollar {
    Dollar {
        amount: dollar.amount * multiplier,
    }
}

#[cfg(test)]
mod money_test {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Dollar { amount: 50 };
        let result1 = times(2, &five);
        assert_eq!(100, result1.amount);
        let result2 = times(3, &five);
        assert_eq!(150, result2.amount);
    }

    #[test]
    fn test_equality() {
        assert_eq!(true, Dollar { amount: 50 } == Dollar { amount: 50 });
        assert_eq!(false, Dollar { amount: 50 } == Dollar { amount: 60 });
    }
}