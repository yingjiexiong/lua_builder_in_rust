use core::panic;
use std::{io::{Read, Bytes}, mem, iter::Peekable, char};

#[derive(Debug)]
pub struct Lex<R:Read>{
    input: Peekable::<Bytes::<R>>,
    head:Token,
}
#[derive(Debug,PartialEq)]
pub enum Token {
    //keywords
    And,    Break,  Do,         Else,   Elseif, End,
    False,  For,    Function,   Goto,   If,     In,
    Local,  Nil,    Not,        Or,     Repeat, Return,
    Then,   True,   Until,      While,
//  +       -       *       /       %       ^       #
    Add,    Sub,    Mul,    Div,    Mod,    Pow,    Len,
//  &       ~       |       <<      >>      //
    BitAnd, BitXor, BitOr,  ShiftL, ShiftR, Idiv,
//  ==      ~=      <=      >=      <       >           =
    Equal,  NotEq,  LesEq,  GreEq,  Less,   Greater,    Assign,
//  (       )       {       }       [       ]           ::
    ParL,   ParR,   CurlyL, CurlyR, SqurL,  SqurR,      DoubColon,
//  ;               :       ,       .       ..          ...
    SemiColon,      Colon,  Comma,  Dot,    Concat,     Dots,


    Integer(i64),
    Float(f64),
    Name(String),
    Strng(Vec<u8>),
    Eos,
}

impl<R:Read> Lex<R> {
   pub fn new(input : R) ->Self{
        Lex {
            input:input.bytes().peekable(),
            head:Token::Eos,
        } 
   }
   pub fn next(&mut self)->Token{
        if self.head == Token::Eos{
            self.do_next()
        }
        else{
            mem::replace(&mut self.head, Token::Eos)
        }
   }

   pub fn peek(&mut self)->&Token {
    // this function can use Some(x).take() to instead
      if self.head == Token::Eos{
        self.head = self.do_next();
      } 
      &self.head
   }
   fn do_next(&mut self)->Token {

        if let Some(ch) = self.next_byte(){
        match ch {
           b' ' | b'\r' | b'\n' | b'\t' => self.do_next(),
           b'+'=>Token::Add,
           b'*'=>Token::Mul,
           b'%'=>Token::Mod,
           b'^'=>Token::Pow,
           b'#'=>Token::Len,
           b'&'=>Token::BitAnd,
           b'|'=>Token::BitOr,
           b'('=>Token::ParL,
           b')'=>Token::ParR,
           b'{'=>Token::CurlyL,
           b'}'=>Token::CurlyR,
           b'['=>Token::SqurL,
           b']'=>Token::SqurR,
           b';'=>Token::SemiColon,
           b','=>Token::Comma,
           b'/'=>self.check_ahead(b'/', Token::Idiv, Token::Div),
           b'='=>self.check_ahead(b'/', Token::Equal, Token::Assign),
           b'~'=>self.check_ahead(b'=', Token::NotEq, Token::BitXor),
           b':'=>self.check_ahead(b':', Token::DoubColon, Token::Colon),
           b'<'=>self.check_ahead2(b'=', Token::LesEq, b'<', Token::ShiftL, Token::Less),
           b'>'=>self.check_ahead2(b'=', Token::GreEq, b'>', Token::ShiftR, Token::Greater),
           b'.'=>match self.read_char() {
                b'.'=>{
                    self.next_byte();
                    if self.read_char() == b'.'{
                        self.next_byte();
                        Token::Dots
                    }
                    else {
                        Token::Concat
                    }
                },
                b'0'..=b'9'=>{
                    self.read_digit_fraction(0)
                },
                _=>{
                    Token::Dot
                },
           },
           b'-'=>{
                if self.read_char() == b'-'{
                    self.next_byte();
                    self.read_comment();
                    self.do_next()
                }
                else {
                    Token::Sub
                }
           },
           b'\''| b'"' => self.read_string(ch),
           b'A'..=b'Z' | b'a'..=b'z' | b'_'=>self.read_name(ch),
           b'0'..=b'9'=>self.read_number(ch),
           b'\0' => Token::Eos,
           _ => panic!("unexpected char: {ch}"),
        }
      }
      else{
        Token::Eos
      }

   }

   fn read_char(&mut self) -> u8{

      match self.input.peek() {
         Some(Ok(ch)) => *ch,
         Some(_) =>panic!("lex read error"),
         None => b'\0',
      }

   }

   fn next_byte(&mut self)->Option<u8>{
    self.input.next().and_then(|r|Some(r.unwrap()))
   }

   fn read_name(&mut self,first:u8)->Token{
        let mut s = String::new(); 
        s.push(first as char);

        loop {
           let ch = self.read_char() as char;
           if ch.is_alphanumeric() || ch == '_'{
            self.next_byte();
            s.push(ch);
           }
           else {
               break;
           }
        } 

        match &s as &str {
            "nil"=>Token::Nil,
            "true"=>Token::True,
            "false"=>Token::False,
            "local"=>Token::Local,
            "and"=>Token::And,
            "break"=>Token::Break,
            "do"=>Token::Do,
            "if"=>Token::If,
            "else"=>Token::Else,
            "elseif"=>Token::Elseif,
            "end"=>Token::End,
            "function"=>Token::Function,
            "goto"=>Token::Goto,
            "for"=>Token::For,
            "in"=>Token::In,
            "not"=>Token::Not,
            "or"=>Token::Or,
            "repeat"=>Token::Repeat,
            "return"=>Token::Return,
            "then"=>Token::Then,
            "until"=>Token::Until,
            "while"=>Token::While,
            _=>Token::Name(s),
        }

   }
   fn read_number(&mut self,first:u8)->Token {
      let mut num = (first - b'0') as i64;

      /*heximal */
      if first == b'0'{
        let second = self.read_char();
        if second == b'x' || second == b'X'{
          return self.read_heximal();
        }
      }

      /*decima */
      loop {
         let ch = self.read_char();
         if let Some(num1) = char::to_digit(ch as char, 10){
            self.next_byte();
            num = num * 10 + num1 as i64;
         }
         else if ch == b'.'{
          return self.read_digit_fraction(num); 
         }
         else if ch == b'e' || ch == b'E'{
          return self.read_number_exp(ch as f64);
         }
         else{
          break;
         }
      } 

      let fcn = self.read_char();
      if(fcn as char).is_alphabetic() || fcn == b'.'{
        panic!("malformat number")
      }

      Token::Integer(num)
   }

   fn read_digit_fraction(&mut self,n:i64)->Token{
      self.next_byte();
      let mut num_i:i64 = 0;
      let mut x = 1.0;
      loop {
        let ch = self.read_char();
        if let Some(num1) = char::to_digit(ch as char,10){
          self.next_byte();
          num_i = num_i * 10 + num1 as i64; 
          x *=  10.0;
        }      
        else{
          break;
        }

      }
      let  num_f = num_i as f64 / x;
      dbg!(&n);
      dbg!(&(num_i));
      dbg!(&num_f);
      Token::Float(n as f64 + num_f)
   }

   fn read_string(&mut self,quote:u8)->Token{

    let mut s = Vec::new();
    loop {
        match self.next_byte().expect("infinished string") {
            b'\n' | b'\0' => panic!("unfinished string") ,
            b'\\'=>todo!("escape"),
            ch if ch == quote => break,
            ch => s.push(ch),
        }    
    } 
    Token::Strng(s)
   }

   fn check_ahead(&mut self,ch:u8,short:Token,long:Token)->Token {

           if self.read_char() == ch{
                // short 
                self.next_byte();
                short
           } 
           else {
                // self.pullback_char();
                long 
           }
   }

   fn check_ahead2(&mut self,ch:u8,short1:Token,ch1:u8,short2:Token,long:Token)->Token{

            let t = self.read_char();
            if t == ch{
                self.next_byte();
                short1
            }
            else if t == ch1{
                self.next_byte();
                short2 
            }
            else {
                // self.pullback_char();
                long
            }
   }

   fn read_comment(&mut self){
    match self.read_char() {
       b'['=>todo!("long comment"),
       _=>{
            loop {
                let ch = self.read_char();
                if ch == b'\n' || ch == b'\0'{
                    break;
                }
            }
       }
    }
   }

   fn read_number_exp(&mut self,_:f64)->Token{
    self.next_byte();
    todo!("lex number exp")
   }

   fn read_heximal(&mut self)->Token{
    self.next_byte();
    todo!("lex number heximal")
   }
}