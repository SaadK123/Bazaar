#![allow(warnings)]
mod ParseInt;
mod ParseFloat;

use std::ffi::{c_ushort, c_void};
use std::io::empty;
use std::io::ErrorKind::Other;
use std::ops::Index;
use std::path::Component::ParentDir;
use std::path::is_separator;
use std::ptr::null;
use std::str::Matches;
use crate::ArithmeticOperator::AddSub;
use crate::ArithmeticOperator::MulDiv;
use crate::ArithmeticOperator::None;
use crate::ParseFloat::parse_float;
use crate::ParseInt::{count_bits_in_value, parse_int};



 const MAX_SIZE_63_BITS:i64 = 9_223_372_036_854_775_807;


fn main() {

    let n = 0.1;

    let n1 = 0.2;

    println!("{}", n + n1);

}


pub fn convert_int_to_u8_arr(val:i128) -> Option<Vec<u8>> {


    let mut size = count_bits_in_value(val);

    let modulo = size % 8;
    if(modulo != 0 ) {
        size += 8 - modulo;
    }


    let mut arr:Vec<u8> = vec![0;size/8];

    let mut index_arr = 0;

    let mut index_bit = 0;


    let mut val_8_bits:u8 = 0;

    for i in 0..128 {

        val_8_bits  += ((val >> i) & 1) as u8 * 2u8.pow(index_bit);

        index_bit += 1;

        if(index_bit == 8) {
            arr[index_arr] = val_8_bits;
            
            index_arr +=1;

            index_bit = 0;

            val_8_bits = 0;
        }

        if(index_arr == size) {
            return Some(arr);
        }
    }

return Option::None;

}
fn read_code(start : i32,code: &str,token :&mut String) {

   token.clear();

    for i in code.chars().skip(start as usize) {

        token.push(i);

        
        if(token == "if") {


        }

    }
}

fn add(first:&mut Vec<char>,second:String) {

    for i in second.chars() {
        first.push(i);
    }
}


fn add_first_with_second_ref(first:&mut Vec<char>, second:&String) {
    for i in second.chars() {
        first.push(i);
    }
}
fn manage_parentheses(code: &str, start: i32,collect:&mut Vec<char>) -> i32 {



    let mut index_count = start;


    for c in code.chars().skip(index_count as usize) {
        if c == '(' {
            let new_start = manage_parentheses(code, start,collect);
            index_count = new_start;
        }

        if c == ')' {
            let real_start = start as usize;
            add(collect,code.chars().skip(real_start).take((index_count - start) as usize).collect());
            return index_count+1;
        }

        index_count +=1;
    }

     -1
}

#[derive(Eq, PartialEq)]
enum ArithmeticOperator {
    MulDiv,
    AddSub,
    None,
}


impl ArithmeticOperator {
    fn find_precedence(symbol:char) -> ArithmeticOperator {

        match  symbol {
            '*' => MulDiv,
            '/' => MulDiv,
            '-' => AddSub,
            '+' => AddSub,
            _ => None
        }
    }
}



enum Sign {
    PLUS,
    MINUS,
    NONE
}

fn execute_arithmetic(content:Vec<char>) -> Option<(String,String)> {
    let mut value: Vec<char> = content;
    for i in 0..2 {
        let mut next_value: Vec<char> = Vec::new();


        let mut first_val: String = String::from("");


        // true   =  +    false = -  NONE = no sign assigned (strings, char , carriers)

        let mut first_sign = Sign::NONE;


        let mut second_sign  = Sign::NONE;


        let mut type_first:String = String::from("");

        let mut type_second:String = String::from("");


        let mut second_val: String = String::from("");

        let mut symbol: char = '\0';

        for (_, val) in value.iter().enumerate() {
            if (*val == ' ') {
                continue;
            }


            let current_val:&mut String;

            let type_curr_val: &mut String;


            match symbol {
                '\0' => {
                    current_val = &mut first_val;

                    type_curr_val = &mut type_first;
                }

                _ => {
                    current_val = &mut second_val;
                    type_curr_val = &mut type_second;
                }
            }



            if ((*val).is_alphanumeric()) {
                if (*type_curr_val == "\0") {
                    *type_curr_val = "i".to_string();
                }
                continue;
            }

            if(*val == '.' && type_curr_val == "i") {
                *type_curr_val = "f".to_string();
                continue;
            }

            if((*val).is_alphabetic() || *val == '_') {
                if(*type_curr_val == "\0") {
                 *type_curr_val = "var".to_string();
                }
            }



            
            let arithmetic_op = ArithmeticOperator::find_precedence(*val);

            if(arithmetic_op != None) {



                let is_prioritized:bool = match arithmetic_op {
                    AddSub => {
                        if i == 1 {true} else {false}
                    }

                    _ => {
                        if i == 0 {true} else {false}
                    }
                };


            }
        }
    }
    // a changer
    return Option::None
}



 pub fn execute_arithmetic_float_int(first:i64,second:i64) -> (i128) {



     return (0);
 }





    pub fn try_get_number (val:usize) -> Option<usize> {
         if(val <= 57 && val  >= 48) {
             return Some(val - 48);
         }
         Option::None
    }




  pub fn try_rectify_sign_power(val:char) -> (char,usize) {
      if(val == '+' || val == '-') {
          return (val,1)
      }
      ('+',0)
  }

pub fn try_adding_digit_to_num_i128(power:usize,num:&mut i128,digit:u8) -> bool {
    let value = try_get_number(digit as usize);

    if(value  == Option::None) {
        return false;
    }

    let value:i128 = get_power_of_number(power,&value.unwrap());

    *num  += value;

    true
}



fn  get_power_of_number(power:usize,digit:&usize) -> i128 {
    10i128.pow(power as u32) * *digit as i128
}

pub fn try_convert_i128_to_i64(num:i128) -> Option<i64> {
    if(num <= i64::MAX as i128) {
        return Some(num as i64);
    }
    Option::None
}

pub fn try_convert_i128_to_u64_with_max_63_bits_length(num:i128) -> Option<u64> {


    if(num <= 9_223_372_036_854_775_807) {
       return Some(num as u64);
    }

      Option::None
}






