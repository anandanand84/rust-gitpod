extern crate bigdecimal;
extern crate rand;

  use bigdecimal::BigDecimal;
    use rand::Rng;
    use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
        let a = rng.gen::<f64>();
        let b = rng.gen::<f64>();
        let runs = 100000;
        let start = Instant::now();
        println!("{}", a);

        for n in 1..runs {
            let m = n as f64;
            let _c = a * m;
            let _d = b * m;
        }
        let end = start.elapsed();
        println!("Time taken for {} runs float {:?}", runs, end);
}


