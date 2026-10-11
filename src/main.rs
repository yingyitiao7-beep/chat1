/*基于运算符优先级的的算法，Precedence Climbing,
 本质是一递归下降，通过递归操作，根据运算符的优先级和结合性来确定计算顺序 */

use std::{self, fmt::Display, fs::read_link, io::{self, Write}, iter::Peekable, result, str::Chars, write};

pub type Result<T> = std::result::Result<T, ExprError>;

#[derive(Debug)]
pub enum ExprError {//pud表示这个枚举可以在符合模块可见性规则的其他模块中被访问
    Parse(String),
}

impl std::error::Error for ExprError {}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprError::Parse(s) => write!(f, "{}", s),
        }
    }
}

 #[derive(Debug, Clone, Copy)]
 enum Token {
     Number(i32),
     Jia,
     Jian,
     Cheng,
     Chu,
     Mi,
     LeftParen,
     RightParen,
 }

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {//self借用当前的Token，f需要输出的目标
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Jia => "+".to_string(),
                Token::Jian => "-".to_string(),
                Token::Cheng => "*".to_string(),
                Token::Chu => "/".to_string(),
                Token::Mi => "^".to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => ")".to_string(),
            }
        )
    }
}

const ASSOC_LEFT: i32 = 0;//左右结合的优先级
const ASSOC_RIGHT: i32 = 1;

impl Token {
    fn is_op(&self) -> bool {
        match self {
            Token::Jia | Token::Jian | Token::Cheng | Token::Chu | Token::Mi => true,
            _ => false,
        }
    }

    fn precedence(&self) -> i32 {
    match self {
        Token::Jia | Token::Jian => 1,
        Token::Cheng | Token::Chu => 2,
        Token::Mi => 3,
        _ => 0,
        }
    }

    fn assoc(&self) -> i32 {
    match self {
        Token::Mi => ASSOC_RIGHT,
        _ => ASSOC_LEFT,
        }
    }

    fn compute(&self, l: i32, r: i32) -> Option<i32> {
    match self {
        Token::Jia => Some(l + r),
        Token::Jian => Some(l - r),
        Token::Cheng => Some(l * r),
        Token::Chu => Some(l / r),
        Token::Mi => Some(l.pow(r as u32)),
        _ => None,
        }
    }
}    

struct Tokenizer<'a> {
    tokens :Peekable<Chars<'a>>,//Chars是字符迭代器，在'a生命周期内生效， Peekable是一个迭代器适配器，允许我们在不消耗元素的情况下查看下一个元素
}

impl <'a> Tokenizer<'a> {
    fn new(expr: &'a str) -> Self {
        Self {
            tokens: expr.chars().peekable(),//chars()方法将字符串切片转换为字符迭代器，peekable()方法将其包装为Peekable迭代器
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();
        while let Some(&c) = self.tokens.peek() {
            if c.is_numeric() {
                num.push(c);
                self.tokens.next();
            } else {
                break;
            }
        }

        match num.parse() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => None,
        }
    }

    fn scan_op(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Jia),
            Some('-') => Some(Token::Jian),
            Some('*') => Some(Token::Cheng),
            Some('/') => Some(Token::Chu),
            Some('^') => Some(Token::Mi),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}

 impl<'a> Iterator for Tokenizer<'a> {/*这里的for不是循环，而是为Tokenizer<'a>实现Iterator这个特征，他是Rust标准库提供的一个trait，规定他之后，Tokenizer就可以逐个生产元素 */
    type Item = Token;//给Token这个类型起一个别名，让之后用到的值都可以直接调用返回Token类型

    fn next(&mut self ) -> Option<Self::Item> {//&mut self是可变借用当前对象，因为迭代器读取到下一个字符后，内部读取回发生变化
        
        self.consume_whitespace();//跳过输入中的空白字符

        match self.tokens.peek() {//self:当前的Tokenizer tokens:当前在结构体中保存的字符 peek:查看下一个字符

            Some(c) if c.is_numeric() => self.scan_number(),//如果返回数字就进行=>

            Some(_) => self.scan_op(),//如果是符号

            None => return None,//都不是
        }
    }
 }

struct Ecpr<'a> {
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Ecpr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(src).peekable(),
        }
    }

    pub fn eval(&mut self) -> Result<i32> {
        let result =  self.compute_expr(1)?;
        if self.iter.peek().is_some() {//如果有未处理的，返回错误
            return Err(ExprError::Parse("Unexpected end of expr".into()));
        }
        Ok(result)
    }

    fn compute_atom(&mut self) -> Result<i32> {//开始判断
        match self.iter.peek() {
            Some(Token::Number(n)) => {//如果是数字，直接返回
                let val = *n;
                self.iter.next();
                return Ok(val);
            }

            Some(Token::LeftParen) => {//如果是括号，开始计算
                self.iter.next();
                let result = self.compute_expr(1)?;
                match self.iter.next() {
                    Some(Token::RightParen) => (),
                    _ => return Err(ExprError::Parse("Unexpected cahraacter" .into())),
                }
                return Ok(result);
            }
            _ => {
                return Err(ExprError::Parse(
                    "Expecting a number or left parenthesis" .into()
                ))
            }
        }
    }

    fn compute_expr(&mut self, min_prec: i32) -> Result<i32> {
        let mut atom_lhs = self.compute_atom()?;//开始评估第一个标记

        loop {
            let cur_token = self.iter.peek();
            if cur_token.is_none() {
                break;
            }

            let token = *cur_token.unwrap();

            if !token.is_op() || token.precedence() < min_prec {
                break;
            }

            let mut next_prec = token.precedence();
            if token.assoc() == ASSOC_LEFT {
                next_prec += 1;
            }

            self.iter.next();

            let atom_rhs = self.compute_expr(next_prec)?;

            match token.compute(atom_lhs, atom_rhs) {
                Some(res) => atom_lhs = res,
                None => return Err(ExprError::Parse("Unexpected character".into())),
            } 
        }

        Ok(atom_lhs)
    }
}

fn main() {
    println!("输入算式");
    
    io::stdout().flush().unwrap();

    let mut src = String::new();

    io::stdin()
        .read_line(&mut src)
        .expect("读取失败");

    let mut expr = Ecpr::new(&src);

    let result = expr.eval();

    match result {
        Ok(vlaue) => println!("结果 = {}", vlaue),
        Err(e) => println!("错误{}",e),
    }

}