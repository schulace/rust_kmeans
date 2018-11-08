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
  let mut runner_seq_2 = runner_par.clone();
  let mut runner_par_2 = runner_par.clone();
  let mut runner_seq = runner_par.clone();
  //println!("constructed new runner in {:?}", cons_time);
  let (time_par, iterations_par) = benchmark(|| runner_par.run_par());
  let (time_seq, _terations_seq) = benchmark(|| runner_seq.run_seq());
  let (time_seq2, _erations_se2) = benchmark(|| runner_seq_2.run_seq_2());
  let (time_par2, _iter_par2)    = benchmark(|| runner_par_2.run_par_2());
  println!("time for seq1: {:?}, time for seq2: {:?}", time_seq, time_seq2);
  //eprintln!("points, dimensions, iterations, time_par, time_seq");
  println!("time for par1: {:?}, time for par2: {:?}", time_par, time_par2);
  println!("{}, {}, {}, {}, {:?}, {:?}, {:?}",
    cfg.total_points, cfg.spatial_dimensions, cfg.k, iterations_par, cons_time, time_par, time_seq);
}
