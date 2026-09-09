use std::arch::x86_64::_mm_undefined_si128;
use std::cmp::max;
use std::ffi::c_ushort;
use std::pin::pin;
use std::process::id;
use std::sync::atomic::AtomicI64;
use crate::{add, add_first_with_second_ref, try_convert_i128_to_i64, try_convert_i128_to_u64_with_max_63_bits_length, try_rectify_sign_power, MAX_SIZE_63_BITS};
use crate::ParseInt::{count_bits_in_value, counts_number_of_decimal_power, parse_int, parse_int_explicit, parse_int_with_binary};

use bitvec::prelude::*;

fn find_point_position(start:usize,bytes_values:&[u8]) -> Option<usize> {




    let mut point_pos:usize = start;

    let mut index:usize  = start;
    for i in start..bytes_values.len() {

        index +=1;

        let char_val = bytes_values[i];

        if(char_val != b'.') {
           continue;
        }


        if(index -1 == start || index-1 == bytes_values.len()-1 || point_pos != start) {
            return None;
        }

        point_pos =  index-1;
    }

    if(point_pos == start) {
        return Some(0);
    }


     Some(point_pos)
}



pub fn parse_float(val:String) -> Option<(i64,u64)> {

    if(val.is_empty()) {
        return None
    }


    let u8_bytes_arr = val.as_bytes();

    let values = try_rectify_sign_power(u8_bytes_arr[0] as char);



    let point_position = find_point_position(values.1,&u8_bytes_arr);


    if point_position == None {
        return None
    }

    let point_position = point_position?;



    let before_point_raw_i128  = parse_int_explicit(values.1,&u8_bytes_arr,point_position,values.0,0);


    let after_point_raw_i128 = parse_int_with_binary(u8_bytes_arr,point_position+1,u8_bytes_arr.len());

    
    if before_point_raw_i128.is_none() || after_point_raw_i128.is_none() {
        return None
    }

    let before_point = try_convert_i128_to_i64(before_point_raw_i128.unwrap());

    let after_point = try_convert_i128_to_u64_with_max_63_bits_length(after_point_raw_i128.unwrap());


    if(before_point.is_none() || after_point.is_none()) {
        return None;
    }

    Some((before_point?,after_point?))
}

fn un_parse_float_decimal(float_decimal:u64) -> (i64,usize) {
    let mut offset_to_last_1:Option<usize> = None;
    let mut offset_to_previous_1:Option<usize> = None;

    for i   in 0..63 {
        let is_one = float_decimal  >> i & 1 == 1;

        if(is_one) {
            offset_to_previous_1 = offset_to_last_1;
            offset_to_last_1 = Some(i);
        }

    }

    if(offset_to_previous_1 == None) {
        return (0,0);
    }

    let number_of_points =  (offset_to_last_1.unwrap() - offset_to_previous_1.unwrap() -1);

    let mut val:u64 = 0;


    for i in 0..=offset_to_previous_1.unwrap() {
        let is_one  = float_decimal << i & 1 == 1;

        if(is_one) {
            val += 1 << i;
        }

    }
    (val as i64,number_of_points)
}


// a float is also a number so it should be able to operate with an int
pub fn operate_math_on_float_int(float:(i64,u64),int:i128,order:bool,symbol:char) -> (i64,u64) {

    let decimal_part_with_offset = un_parse_float_decimal(float.1);

      let i64_number = try_convert_i128_to_i64(int).unwrap();


    if(order) {
      let before_comma_part:i64 = float.0;




    }



    // todo
    return (0,0);
}


fn operate_arithmetic(symbol:char,float_1:(i64,i64),float_2:(i64,i64)) -> Option<(i64,i64)> {
    let mut integral_part:Option<i64>;

    let mut float_part:u64 = 0;


    let mut add_val:u64 = 0;

    if(symbol == '+') {
       return operate_plus_op(float_1,float_2);
    }else if(symbol == '-') {

    }

    return None;
}


/*
 10,52 * 20,12



 4      4
 1052 * 2012


 8

 */


/// reput in u64
fn operate_multiplication_op(first:(i64,i64),second:(i64,i64)) {

    // find bitshift of first in decimal

    let offset_first = counts_number_of_decimal_power(first.1 as i128);

    let first_shifted = shift_two_ints(first.0,first.1);


    // find bitshift of second in decimal


    let second_shifted = shift_two_ints(second.0,second.1);

    let offset_second = counts_number_of_decimal_power(second.1 as i128);


    let max_offset = offset_first + offset_second;



    let val =  first_shifted * second_shifted;




}



// todo use arr &[u8] instead of normal i128

pub fn sub_int(mut val:i128,start:usize,max_power:usize) -> i128 {



    let mut new_val:i128 = 0;

    let number_of_powers = counts_number_of_decimal_power(val)-1;

    if(number_of_powers == 0) {
        return 0;
    }



    if(max_power > number_of_powers) {
        return 0;
    }


    let mut index = start;
    for i in  start..=number_of_powers {


        let curr_val = val / 10i128.pow((number_of_powers - i) as u32) / index as i128;


        new_val += curr_val;
        index+=1;
    }


    return new_val;

}

pub fn shift_two_ints(first:i64,second:i64) -> i128 {
    let mut num:i128 = 0;

   let mut bits_count =  count_bits_in_value(first as i128);

    for  i in 0..bits_count {
       num << (first << i);
    }

    bits_count  = count_bits_in_value(second as i128);

    for i in 0..bits_count {
        num << (second << i);
    }

    num
}



fn operate_minus_op(first:(i64,i64), mut second:(i64, i64)) -> Option<(i64, i64)>{
    if(second.0 < 0) {
        second.0 = second.0 * -1;
        return operate_plus_op(first,second);
    }


    let mut integral_part =
        try_convert_i128_to_i64(first.0 as i128 - second.0 as i128)?;

    let mut floats:i64 = first.1 - second.1;

    if(floats < 0) {
        floats = ((floats + MAX_SIZE_63_BITS));
        integral_part -= 1;
    }

    Some((integral_part,floats))
}


fn operate_plus_op(first:(i64,i64),second:(i64,i64)) -> Option<(i64,i64)> {
   let  mut integral_part = try_convert_i128_to_i64((first.0 + second.0) as i128);



    let mut add:i128 = (first.1 + second.1) as i128;


    if(add > MAX_SIZE_63_BITS as i128) {
        add -=  MAX_SIZE_63_BITS as i128;
        integral_part =  try_convert_i128_to_i64((integral_part.unwrap() + 1 ) as i128);

    }


    if(integral_part == None) {
        return None;
    }

     Some((integral_part.unwrap(),add as i64))
}


fn get_max(first:u64,second:u64) -> u64 {
    if(first > second) {
        return first;
    }
     second
}

