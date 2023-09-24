use std::{fs::File, io::{Read, Seek}};

#[derive(Debug)]
pub struct Lex{
    input: File,
}
#[derive(Debug,PartialEq)]
pub enum Token {
    //keywords
    And,    Break,  Do,         Else,   Elseif, End,
    False,  For,    Function,   Goto,   If,     In,
    Local,  Nil,    Not,        Or,     Repeat, Return,
    Then,   True,   Until,      While,
//  +       -       *       /       %       ^       #
    Add,    Sub,    Nul,    Div,    Mod,    Pow,    Len,
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
    Strng(String),
    Eos,
}

impl Lex {
   pub fn new(input : File) ->Self{
       Lex { input, } 
   }
   pub fn next(&mut self)->Token {

        let ch = self.read_char();
        match ch {
           ' ' | '\r' | '\n' | '\t' => self.next(),
           '\0' => Token::Eos,
           '('=>Token::ParL,
           ')'=>Token::ParR,
           '='=>Token::Assign, 
           '"' => {
                let mut s = String::new();
                loop {
                    match self.read_char() {
                       '\0' => panic!("") ,
                       '"' => break,
                       ch => s.push(ch),
                    }    
                } 
                Token::Strng(s)
           } 

           'A'..='Z' | 'a'..='z' | '_'=>self.read_name(ch),
           '0'..='9'=>self.read_number(ch),
           _ => panic!("unexpected char: {ch}"),
        }

   }

   fn read_char(&mut self) -> char{

        let mut buf:[u8;1] = [0];
        if self.input.read(&mut buf).unwrap() == 1{
            buf[0] as char
        }
        else {
            '\0'
        }
   }

   fn pullback_char(&mut self){
      self.input.seek(std::io::SeekFrom::Current(-1)).unwrap();
   }

   fn read_name(&mut self,first:char)->Token{
                
        let mut s = first.to_string();
        loop {
           match self.read_char() {
            '\0'=>break,
            '_' =>s.push('_'),
            ch if ch.is_alphanumeric()=>s.push(ch),
            _=>{
              self.pullback_char();
              break;
            }
           } 
        } 
        match &s as &str {
            "nil"=>Token::Nil,
            "true"=>Token::True,
            "false"=>Token::False,
            "local"=>Token::Local,
            _=>Token::Name(s),
        }

   }
   fn read_number(&mut self,first:char)->Token {
      let mut num = first.to_digit(10).unwrap() as i64;

      /*decima */
      loop {
         let ch = self.read_char();
         if let Some(num1) = ch.to_digit(10){
            num = num * 10 + num1 as i64;
         }
         else if ch == '.'{
          return self.read_digit_fraction(num); 
         }
         else{
          self.pullback_char();
          break;
         }
      } 

      Token::Integer(num)
   }

   fn read_digit_fraction(&mut self,n:i64)->Token{
      let mut num_i:i64 = 0;
      let mut x = 1.0;
      loop {
        let ch = self.read_char();
        if let Some(num1) = ch.to_digit(10){
          num_i = num_i * 10 + num1 as i64; 
          x *=  10.0;
        }      
        else{
          self.pullback_char();
          break;
        }

      }
      let  num_f = num_i as f64 / x;
      dbg!(&n);
      dbg!(&(num_i));
      dbg!(&num_f);
      Token::Float(n as f64 + num_f)
   }
}