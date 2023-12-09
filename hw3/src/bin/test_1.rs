use hw3::birthday_attack::*;

fn fn1(t: i32) -> i16 {
    let mod_t = 1214124;

    let t1 = (t % mod_t) as i16;
    return t1;
}

pub fn main() {
    birthday_attack::do_birthday_attack(fn1);
}
