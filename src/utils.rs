#![allow(non_snake_case)]
use rand::Rng;


pub fn generate_p() -> i32 {
    let mut P = 0;
        loop {
            P = rand::thread_rng().gen_range(10..100);
            if prime_number(P) == 1 {
                return P;
            }
        }
}

pub fn generate_q(p: i32) -> i32{
    let mut Q = 0;
    loop {
        Q = rand::thread_rng().gen_range(10..100);
        if prime_number(Q) == 1 || !(p == q) {
            return Q;
        }
    }
}

pub fn generate_e(fi: i32) -> i32 {
    let mut e = 0;
    loop {
        e = rand::thread_rng().gen_range(2..fi - 1);
        if greatest_common_divisor(e, fi) == 1 {
            return e;
        }
    }
}

fn greatest_common_divisor(mut a: i32,mut b: i32) -> i32 {
    if a < b {
        a = a;
        b = b;
    } else {
        let c = a;
        a = b;
        b = c;
    }
    let mut r;
        while(b!=0){
            r = a % b;
            a = b;
            b = r;
        }
    return a;
}

fn prime_number(P: i32) -> i32 {
    for i in 2..P/2 {
        if P % i == 0 {
            return 0;
        };
    }
    return 1;
}

pub fn Euclid(e:i32, fi:i32) -> i32 {
    let mut a = 0; let mut b = 0;
    if (e > fi) {
        a = e;
        b = fi;
    } else {
        b = e;
        a = fi;
    }
    let mut u = 0; let mut v = 1; let mut m = a;
    while (b!=0) {
        let q = a/b;
        let r = a%b;
        let vk = u - v * q;
        a = b;
        b = r;
        u = v;
        v = vk;
    }
    while (u < 0) {
        u+=m;
    }
    while (u>=m) {
        u-=m;
    }
    return u;
}



/*
#[cfg(test)]

    #[test]
    fn check_password_length() {
        let password = "some_pa\n";
        assert_eq!(password.len(), 8);
        assert!(Registrar::check_length(password).is_ok());
        assert!(Registrar::check_length(password.trim()).is_err())
    }

    #[test]
    fn check_password_is_valid() {
        let password_valid = "somePa@_ss1";
        let invalid_passwords = [
            "somepass",
            "SOMEPASS",
            "Somepass",
            "somePass1",
            "somePa@ ss1",
        ];

        assert!(Registrar::check_symbols(password_valid).is_ok());

        for invalid_pass in invalid_passwords {
            assert!(Registrar::check_symbols(invalid_pass).is_err());
        }
    }

}*/