extern crate rand;
mod myio;
mod kmeans;
use kmeans::*;
use myio::benchmark;
use rand::{FromEntropy};
use rand::rngs::{StdRng};

fn main() {
  let mut all_input_tokens:Vec<f64> = myio::get_tokens_from_stdin();
  let tail = all_input_tokens.split_off(5);
  let cfg = KmeansConfig::from(all_input_tokens.iter().map(|x| *x as u32).collect::<Vec<_>>());
  let data = points_from_vec(tail, cfg.spatial_dimensions);
  let mut randoms = StdRng::from_entropy();
  let (cons_time, mut runner) = benchmark(|| KMeansRunner::new(&cfg, data, &mut randoms));
  println!("constructed new runner in {:?}", cons_time);
  let (time, iterations) = benchmark(|| runner.run());
  println!("ran in {:?} performing {} iterations", time, iterations);
  println!("total time: {:?}", cons_time + time);

  runner.print_clusters();
}
