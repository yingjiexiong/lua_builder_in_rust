
use core::fmt;
use std::rc::Rc;

use crate::vm::ExeState;

const SHORT_STR_MAX: usize=14;
const MID_STR_MAX: usize = 48 - 1;


#[derive(Clone)]
pub enum Value{
   Nil,
   Boolean(bool),
   Integer(i64),
   Float(f64),
  //  String(String) ,
   Function(fn (&mut ExeState)-> i32),

   ShortStr(u8,[u8;SHORT_STR_MAX]),
   MidStr(Rc<(u8,[u8;MID_STR_MAX])>),
   LongStr(Rc<Vec<u8>>),
}


impl fmt::Debug for Value{
   fn fmt(&self,f:&mut fmt::Formatter) ->Result<( ), fmt::Error>{
      match self{
         Value::Nil=>write!(f,"nil"),
         Value::Function(_)=>write!(f,"function"),
         Value::Boolean(b) => write!(f,"{b}"),
         Value::Integer(i) => write!(f,"{i}"),
         Value::Float(n) => write!(f,"{n:?}"),
        //  Value::String(s) => write!(f, "{s}"),
        Value::ShortStr(len, buf) => write!(f, "{}", String::from_utf8_lossy(&buf[..*len as usize])),
        Value::MidStr(s) => write!(f, "{}", String::from_utf8_lossy(&s.1[..s.0 as usize])),
        Value::LongStr(s) => write!(f, "{}", String::from_utf8_lossy(&s)),
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
            // (Value::String(s1), Value::String(s2)) => *s1 == *s2,
            (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1,f2),
            (Value::ShortStr(len1, s1), Value::ShortStr(len2, s2)) => s1[..*len1 as usize] == s2[..*len2 as usize],
            (Value::MidStr(s1), Value::MidStr(s2)) => s1.1[..s1.0 as usize] == s2.1[..s2.0 as usize],
            (Value::LongStr(s1), Value::LongStr(s2)) => s1 == s2,
            (_, _)=> false,
        }
    }
}


fn vec_to_short_mid_str(v: &[u8]) -> Option<Value> {
    let len = v.len();
    if len <= SHORT_STR_MAX {
        let mut buf = [0; SHORT_STR_MAX];
        buf[..len].copy_from_slice(&v);
        Some(Value::ShortStr(len as u8, buf))

    } else if len <= MID_STR_MAX {
        let mut buf = [0; MID_STR_MAX];
        buf[..len].copy_from_slice(&v);
        Some(Value::MidStr(Rc::new((len as u8, buf))))

    } else {
        None
    }
}


impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        vec_to_short_mid_str(&v).unwrap_or(Value::LongStr(Rc::new(v)))
    }
}

impl From<String> for Value {
   fn from(value: String) -> Self {
      // Value::String(value);
       let len = value.len();
       if len <= SHORT_STR_MAX{
            // len [0..14]
            let mut buf = [0;SHORT_STR_MAX];
            buf[..len].copy_from_slice(value.as_bytes());
            Value::ShortStr(len as u8,buf)
       }
       else if len <= MID_STR_MAX {
            // len[15..47]
            let mut buf = [0;MID_STR_MAX];
            buf[..len].copy_from_slice(value.as_bytes());
            Value::MidStr(Rc::new((len as u8,buf)))
       }
       else {
            // len > 47
            Value::LongStr(Rc::new(value.into())) 
       }
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


// ANCHOR: to_vec_string
impl<'a> From<&'a Value> for &'a [u8] {
    fn from(v: &'a Value) -> Self {
        match v {
            Value::ShortStr(len, buf) => &buf[..*len as usize],
            Value::MidStr(s) => &s.1[..s.0 as usize],
            Value::LongStr(s) => s,
            _ => panic!("invalid string Value"),
        }
    }
}

impl<'a> From<&'a Value> for &'a str {
    fn from(v: &'a Value) -> Self {
        std::str::from_utf8(v.into()).unwrap()
    }
}

impl From<&Value> for String {
    fn from(v: &Value) -> Self {
        String::from_utf8_lossy(v.into()).to_string()
    }
}