mod myio;
mod kmeans;
use kmeans::*;

fn main() {
  let mut all_input_tokens:Vec<f64> = myio::get_tokens_from_stdin();
  let tail = all_input_tokens.split_off(5);
  let cfg = KmeansConfig::from(all_input_tokens.iter().map(|x| *x as u32).collect::<Vec<_>>());
  let data = KMeansData::new(tail, &cfg);

}
