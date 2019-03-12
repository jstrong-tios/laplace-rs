#[macro_use]
extern crate itertools;

use statrs::distribution::{Exponential, Normal, Continuous};
use statrs::statistics::Statistics;
use statrs::prec;
use rand::thread_rng;
use rand::distributions::Distribution;

fn main() {
    let mut rng = thread_rng();
    let mut rng2 = thread_rng();
    let mu = 0.0;
    let sigma = 1.25;
    let r = Exponential::new(sigma).unwrap();
    let n = Normal::new(0.0, 1.0).unwrap();
    let xs = r.sample_iter(&mut rng).take(1024).collect::<Vec<f64>>();
    let ys = r.sample_iter(&mut rng).take(1024).collect::<Vec<f64>>();
    let zs = n.sample_iter(&mut rng).take(1024).collect::<Vec<f64>>();
    let xs = itertools::izip!(xs.iter(), ys.iter(), zs.iter()).map(|(&x, &y, &z)| (x - y) * z).collect::<Vec<f64>>();

    macro_rules! stat {
        ($attr:ident) => {
            println!("{}: {}", stringify!($attr), (&xs[..]).$attr());
        }
    }

    stat!(min);
    stat!(max);
    stat!(mean);
    stat!(variance);
    stat!(std_dev);
}
