use rand::Rng;
struct rand_range{
    p1s:i32,
    p1l:i32,
    p2s:i32,
    p2l:i32
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

fn genarate_random_prime_num(){
    let mut rand = rand::thread_rng();
    let p_rand_range = rand_range{
        p1s:7000,
        p1l:rand.gen_range(7001..10000),
        p2s:7000,
        p2l:rand.gen_range(7001..10000)
    };
    let p1_rand:i32 = rand.gen_range(p_rand_range.p1s..p_rand_range.p1l);
    let p2_rand:i32 = rand.gen_range(p_rand_range.p2s..p_rand_range.p2l);
    let p1:i32 = generate_prime_num(p1_rand as usize);
    let p2:i32 = generate_prime_num(p2_rand as usize);
    println!("{}",p1);
    println!("{}",p2);
}

fn main() {
    genarate_random_prime_num();
}
