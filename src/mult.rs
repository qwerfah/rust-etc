extern crate test;

pub fn bin_mult(a: u32, b: u32) -> u32 {
    let mut curr_pow = a;
    let mut acc: u32 = 0;

    for i in 0..i32::BITS {
        let bit = b >> i & 1;
        if bit == 1 {
            acc += curr_pow;
        }
        curr_pow <<= 1;
    }

    acc
}

pub fn std_mult(a: u32, b: u32) -> u32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::prelude::*;
    use test::Bencher;

    #[test]
    fn mult_test() {
        let mut rng = rand::thread_rng();
        for i in 0..1000 {
            let a: u32 = rng.gen_range(0..100);
            let b: u32 = rng.gen_range(0..100);
            assert_eq!(bin_mult(a, b), std_mult(a, b));
        }
    }

    #[bench]
    fn bin_mult_bench(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let a: u32 = rng.gen_range(0..1000);
            let b: u32 = rng.gen_range(0..1000);
            bin_mult(a, b)
        });
    }

    #[bench]
    fn std_mult_bench(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            let a: u32 = rng.gen_range(0..1000);
            let b: u32 = rng.gen_range(0..1000);
            bin_mult(a, b)
        });
    }
}
