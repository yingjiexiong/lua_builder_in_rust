use std::collections::HashMap;

use crate::{value::Value, parse::ParseProto, byte_code::ByteCode};


#[derive(Debug)]

pub struct  ExeState {
   globals: HashMap<String,Value>,
   stack: Vec::<Value>, 
   func_index :usize,
}


impl ExeState {
   pub fn new() ->Self{
      let mut globals = HashMap::new();
      globals.insert(String::from("print"), Value::Function(lib_print));

      ExeState { globals, stack: Vec::new(), func_index:0,}
   }
   
   pub fn execute(&mut self,proto:&ParseProto) {
    for code in proto.byte_codes.iter(){
      match *code {
         ByteCode::GetGlobal(dst,name, )=>{
            let name = &proto.constants[name as usize];
            if let Value::String(key) = name{
               let v = self.globals
                                             .get(key)
                                             .unwrap_or(&Value::Nil)
                                             .clone();
               self.set_stack(dst,v);
            }
            else {
                panic!("invalid global key :{name:?}");
            }
         }
        ByteCode::LoadConst(dst, c) =>{
            let v = proto.constants[c as usize].clone();
            self.set_stack(dst,v);
        } 

        ByteCode::LoadNil(det) => self.set_stack(det, Value::Nil),
        ByteCode::LoadBool(dst, bol) => self.set_stack(dst, Value::Boolean(bol)),
        ByteCode::LoadInt(dst, i) => self.set_stack(dst, Value::Integer(i.into())),
        ByteCode::Call(func, _) => {
            self.func_index = func as usize;
            let func = &self.stack[self.func_index];
            if let Value::Function(f) = func{
               f(self);
            }
            else {
                panic!("invalid function: {func:?}");
            }
        }
        ByteCode::Move(dst, ic) => {
          let v = self.stack[ic as usize].clone();
          self.set_stack(dst, v)
        }
        _=>panic!("expected byte code"),
      }
    }
   } 

   fn set_stack(&mut self,dst:u8,v:Value){
      let dst = dst as usize;
      match dst.cmp(&self.stack.len()) {
        std::cmp::Ordering::Less =>self.stack[dst] = v,
        std::cmp::Ordering::Equal => self.stack.push(v),
        std::cmp::Ordering::Greater => panic!("fall in stack"),
    }
   }
}


fn lib_print(state: &mut ExeState) -> i32{
   println!("{:?}",state.stack[state.func_index + 1]);
   0
}

