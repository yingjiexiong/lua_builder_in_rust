use std::{fs::File, io::{Read, Seek}, mem};

#[derive(Debug)]
pub struct Lex{
    input: File,
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
    Strng(String),
    Eos,
}

impl Lex {
   pub fn new(input : File) ->Self{
        Lex {
            input,
            head:Token::Eos 
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

        let ch = self.read_char();
        match ch {
           ' ' | '\r' | '\n' | '\t' => self.do_next(),
           '+'=>Token::Add,
           '*'=>Token::Mul,
           '%'=>Token::Mod,
           '^'=>Token::Pow,
           '#'=>Token::Len,
           '&'=>Token::BitAnd,
           '|'=>Token::BitOr,
           '('=>Token::ParL,
           ')'=>Token::ParR,
           '{'=>Token::CurlyL,
           '}'=>Token::CurlyR,
           '['=>Token::SqurL,
           ']'=>Token::SqurR,
           ';'=>Token::SemiColon,
           ','=>Token::Comma,
           '/'=>self.check_ahead('/', Token::Idiv, Token::Div),
           '='=>self.check_ahead('/', Token::Equal, Token::Assign),
           '~'=>self.check_ahead('=', Token::NotEq, Token::BitXor),
           ':'=>self.check_ahead(':', Token::DoubColon, Token::Colon),
           '<'=>self.check_ahead2('=', Token::LesEq, '<', Token::ShiftL, Token::Less),
           '>'=>self.check_ahead2('=', Token::GreEq, '>', Token::ShiftR, Token::Greater),
           '.'=>match self.read_char() {
                '.'=>{
                    if self.read_char() == '.'{
                        Token::Dots
                    }
                    else {
                        self.pullback_char();
                        Token::Concat
                    }
                },
                '0'..='9'=>{
                    self.pullback_char();
                    self.read_digit_fraction(0)
                },
                _=>{
                    self.pullback_char();
                    Token::Dot
                },
           },
           '-'=>{
                if self.read_char() == '-'{
                    self.read_comment();
                    self.do_next()
                }
                else {
                    self.pullback_char();
                    Token::Sub
                }
           },
           '\''| '"' => self.read_string(ch),
           'A'..='Z' | 'a'..='z' | '_'=>self.read_name(ch),
           '0'..='9'=>self.read_number(ch),
           '\0' => Token::Eos,
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

   fn read_string(&mut self,quote:char)->Token{

    let mut s = String::new();
    loop {
        match self.read_char() {
            '\n' | '\0' => panic!("unfinished string") ,
            '\\'=>todo!("escape"),
            ch if ch == quote => break,
            ch => s.push(ch),
        }    
    } 
    Token::Strng(s)
   }

   fn check_ahead(&mut self,ch:char,short:Token,long:Token)->Token {

           if self.read_char() == ch{
                short 
           } 
           else {
                self.pullback_char();
                long 
           }
   }

   fn check_ahead2(&mut self,ch:char,short1:Token,ch1:char,short2:Token,long:Token)->Token{

            let t = self.read_char();
            if t == ch{
                short1
            }
            else if t == ch1{
               short2 
            }
            else {
                self.pullback_char();
                long
            }
   }

   fn read_comment(&mut self){
    match self.read_char() {
       '['=>todo!("long comment"),
       _=>{
            loop {
                let ch = self.read_char();
                if ch == '\n' || ch == '\0'{
                    break;
                }
            }
       }
    }
   }
}