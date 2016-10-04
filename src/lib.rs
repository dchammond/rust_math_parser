extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use parser;
    use std::f64;
    #[test]
    fn addition() {
        let input = String::from("1 + 2");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (1+2) as f64);
    }
    #[test]
    fn subtraction() {
        let input = String::from("18 - 4");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (18-4) as f64);
    }
    #[test]
    fn multiplication() {
        let input = String::from("3 * 6");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (3*6) as f64);
    }
    #[test]
    fn divide() {
        let input = String::from("15 / 3");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (15.0/3.0) as f64);
    }
    #[test]
    fn power() {
        let input = String::from("4 ^ 7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (f64::powf(4.0, 7.0)) as f64);
        let input = String::from("4 ^ 7 ^ 8");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (f64::powf(f64::powf(4.0, 7.0), 8.0) as f64));
    }
    #[test]
    fn numerical_negation() {
        let input = String::from("-2");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (-2 as f64));
        let input = String::from("-(2)");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (-2 as f64));
    }
    #[test]
    fn expression_negation() {
        let input = String::from("-(2+3)");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((-1*(2+3)) as f64));
        let input = String::from("15-(-(2+3))");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((15-(-1*(2+3))) as f64));
    }
    #[test]
    fn basic_modulo() {
        let input = String::from("0 % 7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((0 % 7) as f64));
        let input = String::from("1 % 7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((1 % 7) as f64));
    }
    #[test]
    fn negative_modulo() {
        let input = String::from("-1 % 7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((-1 % 7) as f64));
        let input = String::from("(-1) % 7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (((-1) % 7) as f64));
    }
    #[test]
    fn order_of_operations() {
        let input = String::from("4+5*7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((4+(5*7)) as f64));
        let input = String::from("-15/7");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (((-15.0)/7.0) as f64));
        let input = String::from("-2^0.5");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((-1.0*f64::powf(2.0, 0.5)) as f64));
        let input = String::from("(-2)^0.5");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap().is_nan(), ((f64::powf(-2.0, 0.5)) as f64).is_nan());
    }
    #[test]
    fn end() {
        let input = String::from("");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().is_err());
    }
    #[test]
    fn ignore_whitespace() {
        let input = String::from("   12        -  8   ");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (12-8) as f64);
        let input = String::from("142        -9   ");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (142-9) as f64);
        let input = String::from("72+  15");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (72+15) as f64);
        let input = String::from(" 12*  4");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (12*4) as f64);
        let input = String::from(" 50/10");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (50.0/10.0) as f64);
    }
    #[test]
    fn long_add_expressions() {
        let input = String::from("2 -4 +6 -1 -1- 0 +8");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2-4+6-1-1-0+8) as f64);
        let input = String::from("1 -1   + 2   - 2   +  4 - 4 +    6");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (1-1+2-2+4-4+6) as f64);
    }
    #[test]
    fn long_mult_expressions() {
        let input = String::from("2 *4 *6 *1 *1* 0 *8");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2*4*6*1*1*0*8) as f64);
        let input = String::from("1 *1   * 2   * 2   *  4 * 4 *    6");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (1*1*2*2*4*4*6) as f64);
    }
    #[test]
    fn long_mixed() {
        let input = String::from(" 2*3 - 4*5 + 6/3 ");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2.0*3.0-4.0*5.0+6.0/3.0) as f64);
        let input = String::from("2*3*4/8 -   5/2*4 +  6 + 0/3   ");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2.0*3.0*4.0/8.0-5.0/2.0*4.0+6.0+0.0/3.0) as f64);
    }
    #[test]
    fn floating() {
        let input = String::from("10/4");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (10.0/4.0) as f64);
        let input = String::from("5/3");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (5.0/3.0) as f64);
        let input = String::from("3 + 8/5 -1 -2*5");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (3.0+8.0/5.0-1.0-2.0*5.0) as f64);
    }
    #[test]
    fn bad_tokens() {
        let input = String::from("  6 + c");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().is_err());
        let input = String::from("  7 &amp; 2");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().is_err());
        let input = String::from("  %");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().is_err());
    }
    #[test]
    fn syntax_error() {
        let input = String::from(" 5 + + 6");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().is_err());
        let input = String::from(" 5 * / 6");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().is_err());
    }
    #[test]
    fn divide_zero() {
        let input = String::from("5/0");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().unwrap().is_infinite());
        let input = String::from(" 2 - 1 + 14/0 + 7");
        let mut my_parser = parser::Parser::new(input);
        assert!(my_parser.parse().unwrap().is_infinite());
    }
    #[test]
    fn unused_parens() {
        let input = String::from(" (2) ");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2) as f64);
    }
    #[test]
    fn enclosing_parens() {
        let input = String::from("(5 + 2*3 - 1 + 7 * 8)");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (5.0+2.0*3.0-1.0+7.0*8.0) as f64);
        let input = String::from("(67 + 2 * 3 - 67 + 2/1 - 7)");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (67.0+2.0*3.0-67.0+2.0/1.0-7.0) as f64);
    }
    #[test]
    fn sub_expressions() {
        let input = String::from("(2) + (17*2-30) * (5)+2 - (8/2)*4");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((2.0)+(17.0*2.0-30.0)*(5.0)+2.0-(8.0/2.0)*4.0) as f64);
        let input = String::from("(5*7/5) + (23) - 5 * (98-4)/(6*7-42)");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((5.0*7.0/5.0)+(23.0)-5.0*(98.0-4.0)/(6.0*7.0-42.0)) as f64);
    }
    #[test]
    fn nested_parens() {
        let input = String::from("(((((5)))))");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), ((((((5)))))) as f64);
        let input = String::from("(( ((2)) + 4))*((5))");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (((((2.0))+4.0))*((5.0))) as f64);
    }
    #[test]
    fn unbalanced_parens() {
        let input = String::from("2 + (5 * 2");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2.0+(5.0*2.0)) as f64);
        let input = String::from("(((((4))))");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (4.0) as f64);
        let input = String::from("((2)) * ((3");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (2.0*3.0) as f64);
        let input = String::from("((9)) * ((1)");
        let mut my_parser = parser::Parser::new(input);
        assert_eq!(my_parser.parse().unwrap(), (9.0*1.0) as f64);
    }
}
