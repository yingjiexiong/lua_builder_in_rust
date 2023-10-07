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
      println!("{:?}",i);
    }
    proto
}

fn chunk(&mut self){

    loop {
        match self.lex.next() {

           Token::Name(name) =>{
              if self.lex.peek() == &Token::Assign{
                self.assignment(name)
              }
              else {
                self.function_call( name)
              }
           }
           Token::Local=> self.local(),
           Token::Eos=>break,
           t => panic!("unexpected token: {t:?}"),
        }
    }
}

fn local(&mut self){

  let var = if let Token::Name(var) = self.lex.next(){
    var
  }
  else{
    panic!("expected variable");
  };
  
  if self.lex.next() != Token::Assign {
    panic!("expected '=' ");
  }
  self.load_exp(self.locals.len());
  self.locals.push(var);

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
          self.load_exp(iarg);
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
   
fn load_exp(&mut self, dst: usize) {

    let code = match self.lex.next() {
       Token::Strng(s)=>self.load_const(dst, Value::String(s)),
       Token::Name(var)=>self.load_var(dst,var),
       Token::Integer(i)=>{
          if let Ok(ii)=i16::try_from(i) {
            ByteCode::LoadInt(dst as u8, ii)
          }
          else{
            self.load_const(dst, Value::Integer(i))
          }
       }
       Token::True=>ByteCode::LoadBool(dst as u8, true),
       Token::False=>ByteCode::LoadBool(dst as u8, false),
       Token::Float(f)=>self.load_const(dst, Value::Float(f)),
       _=>panic!("invalid argument"),
    };
    self.byte_codes.push(code);
}

fn load_var(&mut self,
            dst: usize, 
            name: String) -> ByteCode {
    if let Some(i) = self.get_local(&name){
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

fn assignment(&mut self,
              name: String
            ){

    self.lex.next();//'='

    if let Some(i) = self.get_local(&name){
      // local variable
      self.load_exp(i);
    }
    else {
      // global variable
      let dst = self.add_const(value::Value::String(name)) as u8;

      let code = match self.lex.next() {
          // from const values
          Token::Nil=>ByteCode::SetGlobalConst(dst, self.add_const(Value::Nil) as u8),
          Token::True=>ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(true)) as u8),
          Token::False=>ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(false)) as u8),
          Token::Integer(i)=>ByteCode::SetGlobalConst(dst, self.add_const(Value::Integer(i)) as u8),
          Token::Float(f)=>ByteCode::SetGlobalConst(dst, self.add_const(Value::Float(f)) as u8),
          Token::Strng(s)=>ByteCode::SetGlobalConst(dst, self.add_const(Value::String(s)) as u8),
          
          // from variable
          Token::Name(n)=>
              if let Some(i) =  self.get_local(&n){
                  ByteCode::SetGlobal(dst, i as u8)
              }
              else{
                  ByteCode::SetGlobalConst(dst, self.add_const(Value::String(n)) as u8)
              }

          _=>panic!("invalid argument"),
      }; 
      
      self.byte_codes.push(code);
    }


}

fn get_local(&self, name: &str) -> Option<usize> {
   self.locals.iter().rposition(|v| v == &name)
}


}








