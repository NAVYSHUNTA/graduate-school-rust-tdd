fn main() {
}

// ここから下が本題
#[derive(PartialEq)]
struct Dollar {
    amount: i32,
}

impl Dollar {
    fn times(&self, multiplier: i32) -> Dollar {
        Dollar {
            amount: self.amount * multiplier,
        }
    }
}

// テストコード
#[cfg(test)]
mod money_test {
    use super::*;
    #[test]
    fn test_multiplication() {
        let fifty = Dollar { amount: 50 };
        let result1 = fifty.times(2);
        assert_eq!(100, result1.amount);
        let result2 = fifty.times(3);
        assert_eq!(150, result2.amount);
    }

    #[test]
    fn test_equality() {
        assert_eq!(true, Dollar { amount: 50 } == Dollar { amount: 50 });
        assert_eq!(false, Dollar { amount: 50 } == Dollar { amount: 60 });
    }
}