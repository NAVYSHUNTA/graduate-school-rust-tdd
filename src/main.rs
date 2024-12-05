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
    fn test_calc1() {
        // 50 ドル札 2 枚の価値は 100 ドル
        let fifty = Dollar { amount: 50 };
        let res1 = fifty.times(2);
        assert_eq!(100, res1.amount);
    }

    #[test]
    fn test_calc2() {
        // 50 ドル札 2 枚の価値は 100 ドル
        // 上記の副作用のテスト
        let fifty = Dollar { amount: 50 };
        let res1 = fifty.times(2);
        assert_eq!(100, res1.amount);
        let res2 = fifty.times(3);
        assert_eq!(150, res2.amount);
    }

    #[test]
    fn test_equals() {
        // 50 ドル札の価値は別の 50 ドル札に等しい
        let fifty = Dollar { amount: 50 };
        assert_eq!(true, fifty == Dollar { amount: 50 });
    }
}