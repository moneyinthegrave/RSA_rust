mod md5;
mod utils;

fn main() {
    let p = utils::generate_p();
    let q = utils::generate_q(p);
    let n = p * q;
    let fi = (p - 1) * (q - 1);
    let e = utils::generate_e(fi);
    let d = utils::Euclid(e, fi);
    let m = 100;
}
