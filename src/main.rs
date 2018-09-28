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
  let randoms = StdRng::from_entropy();
  let (cons_time, mut runner_par) = benchmark(|| KMeansRunner::new(&cfg, data, randoms.clone()));
  let mut runner_seq = runner_par.clone();
  //println!("constructed new runner in {:?}", cons_time);
  let (time_par, iterations_par) = benchmark(|| runner_par.run_par());
  let (time_seq, _terations_seq) = benchmark(|| runner_seq.run_seq());
  //eprintln!("points, dimensions, iterations, time_par, time_seq");
  println!("{}, {}, {}, {}, {:?}, {:?}, {:?}",
    cfg.total_points, cfg.spatial_dimensions, cfg.k, iterations_par, cons_time, time_par, time_seq);
}
