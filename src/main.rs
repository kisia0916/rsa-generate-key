
use std::{collections::btree_map::Range, f32::consts::E, fmt::format};
use num_bigint::{BigUint,ToBigInt};
use num_traits::{pow, Pow};
use rand::Rng;
struct rand_range{
    p1s:i32,
    p1l:i32,
    p2s:i32,
    p2l:i32
}
struct public_key {
    n:i32,
    e:i32
}
struct  private_key{
    d:u64
}
fn generate_prime_num(max:usize)->i32{
    let mut flg_list:Vec<bool> = vec![true;max];
    let mut return_num:i32 = 0;
    flg_list[0] = false;
    flg_list[1] = false;
    for i in 2..(max as f32).sqrt() as i32+1{
        if flg_list[i as usize]{
            for s in (i..max as i32).step_by(i as usize){
                if flg_list[s as usize]{
                    flg_list[s as usize] = false
                }
            }
        }
    }
    for i in (0..max as i32).rev(){
        if flg_list[i as usize]{
            return_num = i;
            break;
        }
    }
    return_num
}

fn genarate_random_prime_num()->(i32,i32){
    let mut rand = rand::thread_rng();
    let mut p_rand_range = rand_range{
        p1s:10,
        p1l:rand.gen_range(11..80),
        p2s:10,
        p2l:rand.gen_range(11..80)
    };
    while p_rand_range.p1l == p_rand_range.p2l {
        p_rand_range.p2l = rand.gen_range(11..80);
    }
    let p1_rand:i32 = rand.gen_range(p_rand_range.p1s..p_rand_range.p1l);
    let p2_rand:i32 = rand.gen_range(p_rand_range.p2s..p_rand_range.p2l);
    let p1:i32 = generate_prime_num(p1_rand as usize);
    let p2:i32 = generate_prime_num(p2_rand as usize);
    (p1,p2)
}

fn generate_public_key(p1:i32,p2:i32)->public_key{
    let mut rand = rand::thread_rng();
    let n = p1*p2;
    let mut e = 0;
    for i in rand.gen_range(10..30)..n{
        if n%i != 0{
            e = i;
            break;
        }
    }
    public_key{n,e}
}
fn generate_private_key(p1:i32,p2:i32,e:i32)->private_key{
    let mut rand = rand::thread_rng();
    let n = ((p1 - 1)*(p2 - 1)) as u64;
    let d= (rand.gen_range(2..3)*n+1)/(e as u64);
    private_key{d}
}
fn find_mod(n: u128,k:u128) -> u128 {
    let mut return_num:u128 = 0;
    for i in 0..n{
        if i%k == n%k && i != 1{
            return_num = i;
            break;
        }
    }
    return_num
}
fn find_mod_2 (n:u64,k:u64)->u64{
    let mut return_num:u64 = 0;
    for i in 0..n{
        if i%k == n%k && i != 1{
            return_num = i;
            println!("{}",i);
            break;
        }
    }
    return_num
}

fn main() {
    let factor = genarate_random_prime_num();
    let public_key = generate_public_key(factor.0,factor.1);
    let private_key = generate_private_key(factor.0, factor.1, public_key.e);
    println!("publicKey:{},{}",public_key.n,public_key.e);
    println!("privateKey:{}",private_key.d);
    let secret_text:u64 = 7;
    let unit_m = BigUint::from(secret_text);
    println!("{}",unit_m);
    let unit_n = BigUint::from(public_key.n as u64);
    let e_mess = public_key.e as usize;
    let encode_text = pow(unit_m,e_mess)%&unit_n;
    let decode_text = pow(encode_text,private_key.d as usize)%&unit_n;
    println!("{}",decode_text);

    // let base = 2;
    // let ex = 200;
    // let test = pow(base,ex);
    // println!("{}",&test);
    // let decode_text:u64 = 
}
