use std::fs::File;

use crate::{value::Value, byte_code::ByteCode, lex::{Lex, Token}};



#[derive(Debug)]
pub struct ParseProto{
    pub constants: Vec::<Value>,
    pub byte_codes: Vec::<ByteCode>,
}


pub fn load(input:File)->ParseProto{
    let mut constants = Vec::new();
    let mut byte_codes = Vec::new();
    let mut locals:Vec<String> = Vec::new();
    let mut lex = Lex::new(input);

    loop {
        match lex.next() {

           Token::Name(name) =>{
              // if lex.next() == Token::Assign{
              //   assignment(&mut byte_codes, &mut constants, &locals, &mut lex, name)
              // }
              // else{
                function_call(&mut byte_codes, &mut constants, &locals, &mut lex, name)
              // }
           }
           Token::Local=>{
                let var = if let Token::Name(var) = lex.next(){
                  var
                }
                else{
                  panic!("expected variable");
                };
                
                if lex.next() != Token::Assign {
                  panic!("expected '=' ");
                }
                load_exp(&mut byte_codes,&mut constants,&locals, lex.next(),locals.len());
                locals.push(var);

           }
           Token::Eos=>break,
           t => panic!("unexpected token: {t:?}"),
        }
    }
    dbg!(&constants);
    dbg!(&byte_codes);
    // dbg!(&locals);
    ParseProto { constants, byte_codes }
}

fn load_exp(byte_codes: &mut Vec<ByteCode>, 
            constants: &mut Vec<Value>, 
            locals:&Vec<String>, 
            token: Token, 
            dst: usize) {

    let code = match token {
       Token::Strng(s)=>load_const(constants, dst, Value::String(s)),
       Token::Name(var)=>load_var(constants,locals,dst,var),
       _=>panic!("invalid argument"),
    };
    byte_codes.push(code);

}

fn load_var(constants: &mut Vec<Value>, 
            locals: &Vec<String>, 
            dst: usize, 
            name: String) -> ByteCode {
    if let Some(i) = locals.iter().rposition(|v| v==&name){
      // local variable
      ByteCode::Move(dst as u8,i as u8)
    }
    else {
      // global variable
        let ic = add_const(constants, Value::String(name));
        ByteCode::GetGlobal(dst as u8, ic as u8)
    }
}

fn load_const(constants: &mut Vec<Value>, arg: usize, i: Value) -> ByteCode {
   ByteCode::LoadConst(arg as u8, add_const(constants, i) as u8) 
}

fn add_const(constants: &mut Vec<Value>, name: Value) -> usize {
    constants.iter()
             .position(|v| v == &name)
             .unwrap_or_else(||{
                  constants.push(name);
                  constants.len()-1
             })
}

// fn assignment(byte_codes: &mut Vec<ByteCode>,
//               constants: &mut Vec<Value>,
//               locals: &Vec<String>,
//               lex:&mut Lex,
//               name: String
//             ){



// }

fn function_call(byte_codes: &mut Vec<ByteCode>,
                 constants: &mut Vec<Value>,
                 locals: &Vec<String>,
                 lex:&mut Lex,
                 name: String
                 ){
  let ifunc = locals.len();
  let iarg = ifunc + 1;
  let ic = add_const(constants,Value::String(name));
  byte_codes.push(ByteCode::GetGlobal(0, ic as u8));

  match lex.next() {
      Token::ParL =>{
          let code = match lex.next() {
              Token::Nil   =>  ByteCode::LoadNil(1),
              Token::True  =>  ByteCode::LoadBool(1,true),
              Token::False =>  ByteCode::LoadBool(1,false),
              Token::Integer(i)=> 
                  // <= 2byte
                  if let Ok(ii) = i16::try_from(i){
                      ByteCode::LoadInt(1,ii)
                  }
                  else{
                      load_const(constants,1,Value::Integer(i))
                  }
              Token::Float(f) => load_const(constants,1,Value::Float(f)),
              Token::Strng(s) => load_const(constants,1,Value::String(s)),
              Token::Name(var)=> load_var(constants, &locals, iarg, var),
              _=> panic!("invalid argument"),
          };
          byte_codes.push(code);

          if lex.next() != Token::ParR{
              panic!("expected ')'");
          }
      } 
      Token::Strng(s)=>{
          let code = load_const(constants,1,Value::String(s));
          byte_codes.push(code);
      }

      _=>panic!("expectd string"),
  }

  byte_codes.push(ByteCode::Call(0, 1));

}