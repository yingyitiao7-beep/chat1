/*基于运算符优先级的的算法，Precedence Climbing,
本质是一递归下降，通过递归操作，根据运算符的优先级和结合性来确定计算顺序 */

use chat1::Ecpr;
use std::{
    self,
    fmt::Display,
    fs::read_link,
    io::{self, Write},
    iter::Peekable,
    result,
    str::Chars,
    write,
};

fn main() {
    println!("输入算式");

    io::stdout().flush().unwrap();

    let mut src = String::new();

    io::stdin().read_line(&mut src).expect("读取失败");

    let mut expr = Ecpr::new(&src);

    let result = expr.eval();

    match result {
        Ok(vlaue) => println!("结果 = {}", vlaue),
        Err(e) => println!("错误{}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::io::SeekFrom::Start;

    use super::*; //父模块

    fn check_expr(src: &str) -> i32 {
        let mut expr = Ecpr::new(src);
        expr.eval().expect("成功")
    }

    #[test]
    fn test_sum() {
        assert_eq!(check_expr("2 + 3"), 5);
    }

    #[test]
    fn test_reduce() {
        assert_eq!(check_expr("8 - 3"), 5);
    }

    #[test]
    fn test_ride() {
        assert_eq!(check_expr("4 * 5"), 20);
    }

    #[test]
    fn test_except() {
        assert_eq!(check_expr("10 / 2"), 5);
    }

    #[test]
    fn test_paren() {
        assert_eq!(check_expr("2 + 3 * 4"), 14);
    }

    #[test]
    fn test_parens() {
        assert_eq!(check_expr("(2 + 3) * 4"), 20);
    }

    #[test]
    fn test_parenss() {
        assert_eq!(check_expr("((2 + 3) * 4) - 5"), 15);
    }

    #[test]
    fn test_exponent() {
        assert_eq!(check_expr("2 ^ 3"), 8);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(check_expr(" 2 + 3 * 4 "), 14);
    }

    #[test]
    fn test_expression() {
        let mut expr = Ecpr::new("(2 + 3");
        assert!(expr.eval().is_err());
    }

    #[test]
    fn test_performance() {
        use std::hint::black_box; //防止编译器吧测试直接优化掉
        use std::time::Instant; //记录时间

        let count = 100_000;

        let start = Instant::now();

        for _ in 0..count {
            //重复执行 100_000 次
            let mut expr = Ecpr::new(black_box("2+3*4"));
            let result = expr.eval().unwrap();
            black_box(result); //性能测试辅助工具
        }

        let parser_time = start.elapsed(); //计算时间从开始到结束经过的时间

        let start = Instant::now();

        for _ in 0..count {
            let result = black_box(2_i32) + black_box(3_i32) * black_box(4_i32);
            black_box(result);
        }

        let arithmet_time = start.elapsed();

        println!("解析并计算耗时: {:?}", parser_time);
        println!("直接计算耗时: {:?}", arithmet_time);
    }
}
