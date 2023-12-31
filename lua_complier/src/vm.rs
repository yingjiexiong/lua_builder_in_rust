use std::{collections::HashMap, io::Read};
use crate::{value::Value , parse::ParseProto, byte_code::ByteCode};


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

      ExeState {  globals, 
                  stack: Vec::new(), 
                  func_index:0,
               }
   }
   
   pub fn execute<R:Read>(&mut self,proto:&ParseProto<R>) {
    for code in proto.byte_codes.iter(){
      match *code {
         ByteCode::GetGlobal(dst,name, )=>{
            let name:&str = (&proto.constants[name as usize]).into();
            let v = self.globals.get(name).unwrap_or(&Value::Nil).clone();
            self.set_stack(dst.into(), v);

         }
         ByteCode::SetGlobal(name, src)=>{
            let name = &proto.constants[name as usize];
            let value = self.stack[src as usize].clone();
            self.globals.insert(name.into(), value);

         }
         ByteCode::SetGlobalConst(name, src)=>{
            let name = &proto.constants[name as usize];
            let value = proto.constants[src as usize].clone();
            self.globals.insert(name.into(), value);

         }
         ByteCode::SetGlobalGlobal(name, src)=>{
            let name = &proto.constants[name as usize];
            let src:&str = (&proto.constants[src as usize]).into();
            let value = self.globals.get(src).unwrap_or(&Value::Nil).clone();
            self.globals.insert(name.into(), value);

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

