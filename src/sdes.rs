use rand::Rng;
use rand::prelude::SliceRandom;
use itertools::Itertools;
use combinatorial::Combinations;
use combinatorial::factorial;
use array2d::Array2D;

//const MAX_10: u16 = 2u16.pow(11) - 1;
//const MAX_8: u8 = 2u8.pow(9) - 1;

//this function performs a p10, p8, etc. where the second argument is the 
//number of bits to be permuted

pub fn p_x(key: u16, p_s: u32) -> u16 {
    let mut permuted: u16 = 0;
    let nums: Vec<u32> = (0..p_s).collect();
    let ten_perms_iter = nums.iter().permutations(nums.len());

    let ten_perms: Vec<Vec<&u32>> = ten_perms_iter.collect();

    if let Some(rand_perm) = ten_perms.choose(&mut rand::thread_rng()) {
        let mut count: u32 = p_s-1;
        
        for pos in rand_perm{
            let bit: u16 = (key >> **pos) & 1;
            if bit == 0 {
                permuted = permuted & !(1 << count);
            } else {
                permuted = permuted | (1 << count);
            }
            if (count > 0) {
                count -= 1;
            }
            
        }
    } else {
        println!("No permutations available");
    }

    return permuted;

}

pub fn p_8(key: u8) -> (u8, Vec<u32>) {
    let mut permuted: u8 = 0;
    let nums: Vec<u32> = (0..8).collect();
    let eight_perms_iter = nums.iter().permutations(nums.len());
    let eight_perms: Vec<Vec<&u32>> = eight_perms_iter.collect();
    let mut initial_perm: Vec<u32> = vec![];
    if let Some(rand_perm) = eight_perms.choose(&mut rand::thread_rng()) {
        let mut count: u32 = 7;
        for pos in rand_perm{
            let bit: u8 = (key >> **pos) & 1;
            if bit == 0 {
                permuted = permuted & !(1 << count);
            } else {
                permuted = permuted | (1 << count);
            }
            if (count > 0) {
                count -= 1;
            }
            initial_perm.push(**pos);
        }
        
    } else {
        println!("No permutations available");
    }
    return (permuted, initial_perm)

}

pub fn p_4(key: u8) -> (u8, Vec<u32>) {
    let mut permuted: u8 = 0;
    let nums: Vec<u32> = (0..4).collect();
    let four_perms_iter = nums.iter().permutations(nums.len());
    let four_perms: Vec<Vec<&u32>> = four_perms_iter.collect();
    let mut initial_perm: Vec<u32> = vec![];
    if let Some(rand_perm) = four_perms.choose(&mut rand::thread_rng()) {
        let mut count: u32 = 3;
        for pos in rand_perm{
            let bit: u8 = (key >> **pos) & 1;
            if bit == 0 {
                permuted = permuted & !(1 << count);
            } else {
                permuted = permuted | (1 << count);
            }
            if (count > 0) {
                count -= 1;
            }
            initial_perm.push(**pos);
        }
        
    } else {
        println!("No permutations available");
    }
    return (permuted, initial_perm)

}

//converts a u8 to a binary string
pub fn u8_to_binary_string(value: u8) -> String {
    format!("{:#010b}", value) //pad with zeros up to length 10
    //length 10 because of leading "0b" sequence + u8

}

pub fn u4_to_binary_string(value: u8) -> String {
    let bin_8 = format!("{:#010b}", value); //pad with zeros up to length 18
    bin_8[6..10].to_string()

}

//converts a u16 to a binary string
pub fn u16_to_binary_string(value: u16) -> String {
    format!("{:#018b}", value) //pad with zeros up to length 18
    //length 18 because of leading "0b" sequence + u16

}

pub fn left_shift(five: &str) -> String {
    let shifted_str_left = format!("{}{}", &five[1..], &five[..1]);
    let l_h: String = shifted_str_left.into();
    l_h
}

//splits
pub fn sp_comb(key: u16) -> u16{
    let binary_string = u16_to_binary_string(key);

    let extracted_key = &binary_string[binary_string.len() - 10..];
    let kbs: String = extracted_key.into();
    let key_bit_string = format!("{:0>10}", kbs);
    let left_five_chars = &key_bit_string[0..5];
    let right_five_chars = &key_bit_string[5..];

    let left_half: String = left_shift(left_five_chars);

    let right_half: String = left_shift(right_five_chars);

    let nks = format!("{}{}", left_half, right_half);
    let new_key_str = format!("{:0>16}", nks);

    let res = u16::from_str_radix(new_key_str.as_str(), 2);
    match res {
        Ok(result) => {
            result
        }
        Err(err) => {
            0u16
        }
    }
}

pub fn n_sp_comb(key: u16) -> u16 {
    let k = sp_comb(key);
    sp_comb(k)
}

pub fn set_bit(num: &mut u16, pos: u32) {
    *num = *num | (1 << pos);
}

pub fn clear_bit(num: &mut u16, pos: u32) {
    *num = *num & !(1 << pos);
}

pub fn get_bit_u8(num: u8, pos: u8) {
    (num >> pos) & 1;
}

pub fn p_10_to_8(key: u16) -> u8 {
    let key1 = 0;
    let key2 = 0;
    let mut k = key;

    let mut combos = Combinations::of_size(0..10, 8);
    let num_combos = factorial(10)/(factorial(8) * factorial(10-8));

    let mut rng = rand::thread_rng();

    let i = rng.gen_range(0..num_combos);

    let res = combos.nth(i);
    let mut com: Vec<i32> = Vec::new();
    match res {
        Some(x) => {
            com = x;
            //getting random permuation of the combination
            let eight_perms_iter = com.iter().permutations(8);
            let eight_perms: Vec<Vec<&i32>> = eight_perms_iter.collect();
            if let Some(rand_perm) = eight_perms.choose(&mut rand::thread_rng()) {
                let mut count: u32 = 7;
                
                for pos in rand_perm{
                    let bit: u16 = (key >> **pos) & 1;
                    //setting/clearing a bit
                    if bit == 0 {
                        clear_bit(&mut k, **pos as u32);
                    } else {
                        set_bit(&mut k, **pos as u32);
                    }
                    if (count > 0) {
                        count -= 1;
                    }
                    
                }
            } else {
                println!("No permutations available");
            }
        },
        None => {},
    };
    
    let binary_string = u16_to_binary_string(k);

    let extracted_key = &binary_string[binary_string.len() - 8..];
    let kbs: String = extracted_key.into();
    let res = u8::from_str_radix(kbs.as_str(), 2);
    match res {
        Ok(result) => {
            result
        },
        Err(err) => {
            0u8
        }
    }


}

pub fn keys(p_x: u16) -> (u8, u8) {
    let q: u16 = sp_comb(p_x);
    let key1 = p_10_to_8(q);
    let r = sp_comb(p_x);
    let g: u16 = sp_comb(r);
    let h: u16 = sp_comb(g);
    let key2 = p_10_to_8(h);
    (key1, key2)
}

pub fn permute_plaintext(plaintext: u8) -> (u8, Vec<u32>) {
    p_8(plaintext)
}

pub fn split_expand(block: u8) -> (u8, Vec<u32>, Vec<u32>) {
    let binary_string = u8_to_binary_string(block);
    let right_half = &binary_string[6..10];
    let nums: Vec<u32> = (0..4).collect();
    let four_perms_iter = nums.iter().permutations(nums.len());
    let four_perms: Vec<Vec<&u32>> = four_perms_iter.collect();
    let right1 = four_perms.choose(&mut rand::thread_rng()).unwrap();
    let right2 = four_perms.choose(&mut rand::thread_rng()).unwrap();
    let mut right1_string = String::from("");
    let mut right2_string = String::from("");
    let mut v1: Vec<u32> = vec![];
    let mut v2: Vec<u32> = vec![];
    for ind in right1.into_iter() {
        let ch = right_half.chars().nth(**ind as usize).unwrap();
        right1_string.push(ch);
        v1.push(**ind);
    }
    for ind in right2.into_iter() {
        let ch = right_half.chars().nth(**ind as usize).unwrap();
        right2_string.push(ch);
        v2.push(**ind);
    }
    let right_num_str = right1_string + &right2_string;
    let res2 = u8::from_str_radix(&right_num_str, 2);
    let mut num2: u8 = 0u8;

    match res2 {
        Ok(result) => {
            num2 = result;
        }
        Err(err) => {}
    }
    (num2, v1, v2)
}

pub fn xor(key1: u8, expanded: u8) -> u8 {
    key1 ^ expanded
}

pub fn s_boxes(key: u8, expanded: u8) -> String {
    let mut s1: Array2D<u8> = Array2D::filled_with(0, 4, 4);
    let mut s2: Array2D<u8> = Array2D::filled_with(0, 4, 4);
    for i in 0..4 {
        for j in 0..4 {
            s1[(i, j)] = rand::thread_rng().gen_range(0..4);
            s2[(i, j)] = rand::thread_rng().gen_range(0..4);
        }
    };
    let x = xor(key, expanded);
    let x_str = u8_to_binary_string(x);
    let left_half = &x_str[2..6];
    let right_half = &x_str[6..10];
    let l1_s1 = left_half.chars().nth(0).unwrap();
    let l2_s1 = left_half.chars().nth(1).unwrap();
    let l3_s1 = left_half.chars().nth(2).unwrap();
    let l4_s1 = left_half.chars().last().unwrap();
    let r1_s2 = right_half.chars().nth(0).unwrap();
    let r2_s2 = right_half.chars().nth(1).unwrap();
    let r3_s2 = right_half.chars().nth(2).unwrap();
    let r4_s2 = right_half.chars().last().unwrap();
    let mut row_s1 = String::from(l1_s1);
    row_s1.push(l4_s1);
    let mut col_s1 = String::from(l2_s1);
    col_s1.push(l3_s1);
    let row_s1_bin = format!("{:0>10}", row_s1);
    let col_s1_bin = format!("{:0>10}", col_s1);
    let r_s1_res = u8::from_str_radix(&row_s1_bin, 2);
    let c_s1_res = u8::from_str_radix(&col_s1_bin, 2);
    let mut r_s1 = 0;
    let mut c_s1 = 0;
    match r_s1_res {
        Ok(r_s1_res) => {
            r_s1 = r_s1_res;
        }
        Err(err) => {}
    }
    match c_s1_res {
        Ok(c_s1_res) => {
            c_s1 = c_s1_res;
        }
        Err(err) => {}
    }
    let s1_res = s1[(r_s1 as usize, c_s1 as usize)];
    let s1_res_str = u8_to_binary_string(s1_res);
    let s1_res_dig = &s1_res_str[8..10];

    let mut row_s2 = String::from(r1_s2);
    row_s2.push(r4_s2);
    let mut col_s2 = String::from(r2_s2);
    col_s2.push(r3_s2);
    let row_s2_bin = format!("{:0>10}", row_s2);
    let col_s2_bin = format!("{:0>10}", col_s2);
    let r_s2_res = u8::from_str_radix(&row_s2_bin, 2);
    let c_s2_res = u8::from_str_radix(&col_s2_bin, 2);
    let mut r_s2 = 0;
    let mut c_s2 = 0;
    match r_s2_res {
        Ok(r_s2_res) => {
            r_s2 = r_s2_res;
        }
        Err(err) => {}
    }
    match c_s2_res {
        Ok(c_s2_res) => {
            c_s2 = c_s2_res;
        }
        Err(err) => {}
    }
    let s2_res = s2[(r_s2 as usize, c_s2 as usize)];
    let s2_res_str = u8_to_binary_string(s2_res);
    let s2_res_dig = &s2_res_str[8..10];

    let mut s_box_output = s1_res_dig.to_string();
    s_box_output += s2_res_dig;
    s_box_output
    
}


pub fn sdes(key: u16, p_text: u8) -> u8 {
    let p: u16 = p_x(key as u16, 10u32);
    let ks = keys(p);
    let plaintext: u8 = 14;
    let scrambled = permute_plaintext(plaintext);
    let ip = u8_to_binary_string(scrambled.0);
    let expanded = split_expand(scrambled.0);
    let v1 = expanded.1;
    let v2 = expanded.2;
    let k1 = s_boxes(ks.0, expanded.0);
    let p4 = u8::from_str_radix(&k1, 2);
    let mut extracted_p4 = 0;
    match p4 {
        Ok(p4) => {
            extracted_p4 = p4;
        }
        Err(err) => {}
    };
    let p4_permuted = p_4(extracted_p4);
    let p4_xor_ip = xor(p4_permuted.0, scrambled.0 >> 4);
    let p4_xor_ip_string = u8_to_binary_string(p4_xor_ip);
    let ip = u8_to_binary_string(scrambled.0);
    let new_lh = &ip[6..10];
    let new_rh = &p4_xor_ip_string[6..10];
    let combined1 = format!("{:0>10}", new_lh.to_string() + new_rh);
    let combined1_res = u8::from_str_radix(&combined1, 2);
    let mut c1 = 0;
    match combined1_res {
        Ok(combined1_res) => {
            c1 = combined1_res;
        }
        Err(err) => {}
    }

    let mut right1_string = String::from("");
    let mut right2_string = String::from("");
    let mut v1: Vec<u32> = vec![];
    let mut v2: Vec<u32> = vec![];
    for ind in v1.into_iter() {
        let ch = new_rh.chars().nth(ind as usize).unwrap();
        right1_string.push(ch);
    }
    for ind in v2.into_iter() {
        let ch = new_rh.chars().nth(ind as usize).unwrap();
        right2_string.push(ch);
    }
    let right_num_str = right1_string + &right2_string;
    let res2 = u8::from_str_radix(&right_num_str, 2);
    let mut num2: u8 = 0u8;
    match res2 {
        Ok(result) => {
            num2 = result;
        }
        Err(err) => {}
    }

    let k2 = s_boxes(ks.1, num2);
    let k2_res = u8::from_str_radix(&k2, 2);
    let mut k2_num = 0;
    match k2_res {
        Ok(result) => {
            k2_num = result;
        }
        Err(err) => {}
    }
    let mut permuted: u8 = 0;
    let mut count = 3;
    for pos in p4_permuted.1 {
        let bit: u8 = (k2_num >> pos) & 1;
        if bit == 0 {
            permuted = permuted & !(1 << count);
        } else {
            permuted = permuted | (1 << count);
        }
        if (count > 0) {
            count -= 1;
        }
    }
    let next_xor = xor(permuted, num2 >> 4);
    let next_xor_str = u8_to_binary_string(next_xor);
    let new_lh_2 = &p4_xor_ip_string[6..10];
    let new_rh_2 = &next_xor_str[6..10];
    let new_str = new_lh_2.to_string() + new_rh_2;
    let final_comb = format!("{:0>10}", new_str);

    let mut fp = vec![0; 8];
    let mut count = 0;
    for pos in scrambled.1 {
        fp[pos as usize] = count;
        count += 1;
    }
    let pre_fp_res = u8::from_str_radix(&k2, 2);
    let mut pre_fp_num = 0;
    match pre_fp_res {
        Ok(result) => {
            pre_fp_num = result;
        }
        Err(err) => {}
    }

    let mut cipher_text = 0;
    let mut c = 7;
    for p in fp {
        let bit: u8 = (pre_fp_num >> p) & 1;
        if bit == 0 {
            cipher_text = cipher_text & !(1 << c);
        } else {
            cipher_text = cipher_text | (1 << c);
        }
        if (c > 0) {
            c -= 1;
        }
    }

    cipher_text
}



