
use core::fmt;
// use std::rc::Rc;

use crate::vm::ExeState;

// const SHORT_STR_MAX: usize=14;
// const MID_STR_MAX: usize = 48 - 1;


#[derive(Clone)]
pub enum Value {
   Nil,
   Boolean(bool),
   Integer(i64),
   Float(f64),
   String(String) ,
   Function(fn (&mut ExeState)-> i32),
  //  ShortStr(u8,[u8;SHORT_STR_MAX]),
  //  MidStr(Rc<(u8,[u8;MID_STR_MAX])>),
  //  LongStr(Rc<Vec<u8>>),
}


impl fmt::Debug for Value{
   fn fmt(&self,f:&mut fmt::Formatter) ->Result<( ), fmt::Error>{
      match self{
         Value::Nil=>write!(f,"nil"),
         Value::Function(_)=>write!(f,"function"),
         Value::Boolean(b) => write!(f,"{b}"),
         Value::Integer(i) => write!(f,"{i}"),
         Value::Float(n) => write!(f,"{n:?}"),
         Value::String(s) => write!(f, "{s}"),
        //  Value::ShortStr(_, _) => todo!(),
        //  Value::MidStr(_) => todo!(),
        //  Value::LongStr(_) => todo!(),
      }
   }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(b1), Value::Boolean(b2)) => *b1 == *b2,
            (Value::Integer(i1), Value::Integer(i2)) => *i1 == * i2,
            (Value::Float(f1), Value::Float(f2)) => *f1 == *f2,
            (Value::String(s1), Value::String(s2)) => *s1 == *s2,
            (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1,f2),
            (_, _)=> false,
        }
    }
}

impl From<String> for Value {
   fn from(value: String) -> Self {
      Value::String(value)
      //  let len = value.len();
      //  if len <= SHORT_STR_MAX{
      //       // len [0..14]
      //       let mut buf = [0;SHORT_STR_MAX];
      //       buf[..len].copy_from_slice(value.as_bytes());
      //       Value::ShortStr(len as u8,buf)
      //  }
      //  else if len <= MID_STR_MAX {
      //       // len[15..47]
      //       let mut buf = [0;MID_STR_MAX];
      //       buf[..len].copy_from_slice(value.as_bytes());
      //       Value::MidStr(Rc::new((len as u8,buf)))
      //  }
      //  else {
      //       // len > 47
      //       Value::LongStr(Rc::new(value.into())) 
      //  }
   } 
}

impl From<i64> for Value{
  fn from(value: i64) -> Self {
     Value::Integer(value) 
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
      Value::Float(value)
  }
}

impl From<bool> for Value{
  fn from(value: bool) -> Self {
      Value::Boolean(value)
  }
}

impl From<()> for Value {
  fn from(_value: ()) -> Self {
      Value::Nil
  }
}

