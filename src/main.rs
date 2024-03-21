
use std::fmt::format;

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
        p1s:10000,
        p1l:rand.gen_range(10001..50000),
        p2s:10000,
        p2l:rand.gen_range(10001..50000)
    };
    while p_rand_range.p1l == p_rand_range.p2l {
        p_rand_range.p2l = rand.gen_range(10001..50000);
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
    let d= (rand.gen_range(100..1000)*n+1)/(e as u64);
    private_key{d}
}
fn find_mod(n: u64,k:u64) -> u64 {
    let list = (0..n).filter(|&x| x % k == n % k).collect::<Vec<_>>();
    list[list.len()-1]
}
fn large_pow (x:u128,y:u128)->u128{
    println!("{},{}",x,y);
    // let mut result:String = format!("{}",x);
    // for i in 0..y-1{
    //     result *= result as u128
    // }
    // println!("{}",result);
    // 23
    23
}
fn main() {
    let factor = genarate_random_prime_num();
    let public_key = generate_public_key(factor.0,factor.1);
    let private_key = generate_private_key(factor.0, factor.1, public_key.e);
    println!("publicKey:{},{}",public_key.n,public_key.e);
    println!("privateKey:{}",private_key.d);
    let secret_text:u64 = 7;
    println!("{}",public_key.e);
    let encoded_text:u64 = find_mod(secret_text.pow(public_key.e as u32), public_key.n as u64);
    println!("{}",encoded_text);
}
