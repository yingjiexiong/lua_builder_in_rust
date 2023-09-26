use std::fs::File;

use crate::{value::{Value, self}, byte_code::ByteCode, lex::{Lex, Token}};



#[derive(Debug)]
pub struct ParseProto{
    pub constants: Vec::<Value>,
    pub byte_codes: Vec::<ByteCode>,
    locals : Vec::<String>,
    lex : Lex,
}

impl ParseProto {
 pub fn load(input:File)->ParseProto{

    let mut proto = ParseProto{
        constants: Vec::new(),
        byte_codes: Vec::new(),
        locals: Vec::new(),
        lex : Lex::new(input),
    };

    proto.chunk();

    print!("proto.constants : {:?}",&proto.constants);
    println!("proto.byte_codes :");
    for i in proto.byte_codes.iter(){
      println!("{:?}",&i);
    }
    proto
}

fn chunk(&mut self){

    loop {
        match self.lex.next() {

           Token::Name(name) =>{
              if self.lex.peek() == &Token::Assign{

              }
              else {
                self.function_call( name)
              }
           }
           Token::Local=>{
                let var = if let Token::Name(var) = self.lex.next(){
                  var
                }
                else{
                  panic!("expected variable");
                };
                
                if self.lex.next() != Token::Assign {
                  panic!("expected '=' ");
                }
                self.load_exp();
                self.locals.push(var);

           }
           Token::Eos=>break,
           t => panic!("unexpected token: {t:?}"),
        }
    }
}

fn function_call(&mut self,
                 name: String
                 ){
  let ifunc = self.locals.len();
  let iarg = ifunc + 1;
  let code = self.load_var(ifunc, name);
  self.byte_codes.push(code);
  match self.lex.next() {
      Token::ParL =>{
          let code = match self.lex.next() {
              Token::Nil   =>  ByteCode::LoadNil(1),
              Token::True  =>  ByteCode::LoadBool(1,true),
              Token::False =>  ByteCode::LoadBool(1,false),
              Token::Integer(i)=> 
                  // <= 2byte
                  if let Ok(ii) = i16::try_from(i){
                      ByteCode::LoadInt(1,ii)
                  }
                  else{
                      self.load_const(iarg,Value::Integer(i))
                  }
              Token::Float(f) => self.load_const(iarg,Value::Float(f)),
              Token::Strng(s) => self.load_const(iarg,Value::String(s)),
              Token::Name(var)=> self.load_var(iarg, var),
              _=> panic!("invalid argument"),
          };
          self.byte_codes.push(code);

          if self.lex.next() != Token::ParR{
              panic!("expected ')'");
          }
      } 
      Token::Strng(s)=>{
          let code = self.load_const(1,Value::String(s));
          self.byte_codes.push(code);
      }

      _=>panic!("expectd string"),
  }

  self.byte_codes.push(ByteCode::Call(ifunc as u8, 1));

}
   
fn load_exp(&mut self) {

    let code = match self.lex.next() {
       Token::Strng(s)=>self.load_const(self.locals.len(), Value::String(s)),
       Token::Name(var)=>self.load_var(self.locals.len(),var),
       Token::Integer(i)=>self.load_const(self.locals.len(), Value::Integer(i)),
       Token::True=>ByteCode::LoadBool(self.locals.len() as u8, true),
       Token::False=>ByteCode::LoadBool(self.locals.len() as u8, false),
       _=>panic!("invalid argument"),
    };
    self.byte_codes.push(code);

}

fn load_var(&mut self,
            dst: usize, 
            name: String) -> ByteCode {
    if let Some(i) = self.locals.iter().rposition(|v| v==&name){
      // local variable
      ByteCode::Move(dst as u8,i as u8)
    }
    else {
      // global variable
        let ic = self.add_const(Value::String(name));
        ByteCode::GetGlobal(dst as u8, ic as u8)
    }
}

fn load_const(&mut self,arg: usize, i: Value) -> ByteCode {
   ByteCode::LoadConst(arg as u8, self.add_const(i) as u8) 
}

fn add_const(&mut self,name: Value) -> usize {
    self.constants.iter()
             .position(|v| v == &name)
             .unwrap_or_else(||{
                  self.constants.push(name);
                  self.constants.len()-1
             })
}
}









// fn assignment(byte_codes: &mut Vec<ByteCode>,
//               constants: &mut Vec<Value>,
//               locals: &Vec<String>,
//               lex:&mut Lex,
//               name: String
//             ){



// }

