extern crate num;
extern crate rand;

use num::bigint::{BigUint, ToBigUint};
use num::traits::One;
use rand::prelude::*;

pub fn is_prime(number: u64) -> bool {
    if number <= 3 && (number == 2 || number == 3) {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    let k = 25;
    let n = number - 1;

    let mut d = n;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = thread_rng();
    for _ in 0..k {
        let a = rng.gen_range(2, n);
        //        let mut x = pow(a, d) % number;
        let mut x = a
            .to_biguint()
            .unwrap()
            .modpow(&d.to_biguint().unwrap(), &number.to_biguint().unwrap());

        if x == One::one() || x == n.to_biguint().unwrap() {
            continue;
        }
        'outer : for _ in 0..s - 1 {
            x = x.modpow(&2.to_biguint().unwrap(), &number.to_biguint().unwrap());

            //            println!("x is {}",x);
            if x == BigUint::from(n) {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

pub fn gen_prime(low: u64, high: u64) -> u64 {
    let mut rng = thread_rng();

    for _ in low..high {
        let rand_num = rng.gen_range(low, high);

        if is_prime(rand_num) {
            return rand_num;
        } else {
            continue;
        }
    }
    1
}

pub fn gen_prime_max(high: u64) -> u64 {
    let mut rng = thread_rng();
    let low;
    if high > 100 {
        low = high - 100;
    } else {
        low = 0;
    }

    for _ in low..high {
        let rand_num = rng.gen_range(low, high);

        if is_prime(rand_num) {
            return rand_num;
        } else {
            continue;
        }
    }
    1
}

pub mod sieve {

    pub fn gen_prime(max : u64) -> Vec<u64> {
        let mut numbers = vec![true ; max as usize];

        numbers[0] = false;

        for i in (2..(max as f64).sqrt() as u64).step_by(2) {
            let mut times = 2;
            while (i + 1) * times <= max {
//                eprintln!("{}", (i + 1) * times);
                numbers[((i + 1) * times - 1) as usize] = false;
                times += 1;
            }
        }

//        eprintln!("{:?}",numbers);

        let mut result = Vec::new();
        result.push(2);
        let mut i = 3;
        for v in (2..numbers.len()).step_by(2) {
            if numbers[v] {
                result.push(i as u64);
//                eprintln!("{}",i);
            }
            i += 2;
        }

        result
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test() {
            let result = gen_prime(1_0000_0000);
//            eprintln!("{:?}", result);
            eprintln!("{}", result.len());
            assert!(false);
        }
    }
}

//slow
fn pow(a: u64, b: u64) -> BigUint {
    //    let mut o = a;
    let mut o = One::one();
    o = o * a;
    eprintln!("{}", o);
    for i in 1..b {
        o *= a;
        eprintln!("{}-i:{}", o, i);
    }
    o
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_prime_test() {
        let number = 2;
        assert!(is_prime(number));

        let number = 3;
        assert!(is_prime(number));

        let number = 20;
        assert!(!is_prime(number));
    }

    #[test]
    fn is_prime_test2() {
        let number = 7;
        assert!(is_prime(number));

        let number = 19;
        assert!(is_prime(number));

        let number = 631;
        assert!(is_prime(number));

        let number = 2083;
        assert!(is_prime(number));

        let number = 27427;
        assert!(is_prime(number));

        let number = 27427;
        assert!(is_prime(number));
    }

    #[test]
    fn pow_test() {
        let a = 2;
        let o = pow(a, 2);
        let n = 4.to_biguint().unwrap();
        assert_eq!(o, n);
    }

    #[test]
    fn time_test() {
        let min: u64 = 13835058055282163000;
        let max: u64 = 13835058055282164000;
        let mut count = 0;
        let mut a;
        let mut is_prime_flag;
        for number in min..max {
            a = 2;
            is_prime_flag = true;
            while a * a < number && is_prime_flag {
                if number % a == 0 {
                    is_prime_flag = false;
                }
                a += 1;
            }
            if is_prime_flag {
                count += 1;
            }
        }
        eprintln!("{}", count);
        assert!(false);
    }

    #[test]
    fn time_test2() {
        let number = 922_3372_0368_5477_5807;
        assert!(is_prime(number));
    }

    #[test]
    fn time_test3() {
        //        let min : u64 = 9223372036854776000;
        let min: u64 = 13835058055282163000;
        let max: u64 = 13835058055282164000;
        let mut count = 0;
        for number in min..max {
            if is_prime(number) {
                count += 1;
            }
        }
        eprintln!("{}", count);
        assert!(false);
    }

    #[test]
    fn gen_prime_test() {
        let low = 10_0000;
        let high = 20_0000;
        let prime = gen_prime(low, high);

        eprintln!("prime is {}", prime);
        assert!(false);
        //        13291， 11783， 10909
        //        135089,
    }

    #[test]
    fn gen_prime_max_test() {
        let high = 2000_0000;
        let prime = gen_prime_max(high);

        eprintln!("prime is {}", prime);
        assert!(false);
        //        199931, 1999969, 19999999
    }

}
