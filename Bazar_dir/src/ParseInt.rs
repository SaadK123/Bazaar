use std::path::is_separator;
use std::time::TryFromFloatSecsError;
use crate::{get_power_of_number, try_adding_digit_to_num_i128, try_convert_i128_to_u64_with_max_63_bits_length, try_get_number, try_rectify_sign_power};



pub fn pre_manage_parse_int(value:&[u8]) -> Option<(char,usize)> {

    if(value.is_empty()) {
        return None;
    }
    let values = try_rectify_sign_power(value[0] as char);

    Some(values)
}



pub fn parse_int(value:&[u8]) -> Option<i128> {
    let values_helper = pre_manage_parse_int(value);
    

parse_int_explicit(values_helper.unwrap().1,value,value.len(),
                   values_helper.unwrap().0,0)

}







pub fn parse_int_with_binary(value:&[u8],start:usize,stop:usize) -> Option<i128> {

    let mut num:i128 = 0;

    let mut number_of_points:usize = 0;


    let mut end = false;

    for i in start..stop {

        let pos  = i;

        let val = value[pos];

        let val  =  try_get_number(val as usize);

        let val = val?;

        if(val > 0) {

            if(!end) {
                end = !end;
            }

            num += get_power_of_number(stop-1-i,&val);

        }else if(!end) {
            number_of_points+=1;
        }
    }


    let bits_shift = count_bits_in_value(num) + number_of_points;

    let binary_power_shift = 1 << bits_shift;

    num += binary_power_shift;

    Some(num)
}


pub fn count_bits_in_value(mut val:i128) -> usize {

    let mut bits = 0;
    while(val != 0) {
       val = val /2;
        bits+=1;
    }
    bits
}

pub fn counts_number_of_decimal_power(mut num:i128) -> usize {
    let mut power = 1;

    while(num >= 10) {
        num = num /10;
        power += 1;
    }

    power as usize
}



pub fn parse_int_explicit(start:usize, value:&[u8], stop:usize, symbol:char, initial_val:i128) -> Option<i128> {

    let mut num:i128 = initial_val;
    
    for i in start..stop {
        
      let power = stop-i-1;


        
      let has_worked  = try_adding_digit_to_num_i128(power,&mut num,value[i]);
        
      if(!has_worked) {
          return None;
      }
        
    }
    
    if(symbol == '-') {
        num = -num;
    }
    
    Some(num)
}





