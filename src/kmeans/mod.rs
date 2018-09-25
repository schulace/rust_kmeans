extern crate rand;
use std::rc::Rc;
use kmeans::rand::prelude::*; //this feels really weird
use myio::vec_to_rc_vec;

#[derive(Debug, Clone)]
pub struct KmeansConfig {
  pub total_points: u32,
  pub spatial_dimensions: u32, //dimensions for the algorithm
  pub k: u32,
  pub max_iterations: u32,
  pub has_name: bool //always false
}

#[derive(Debug, Clone)]
pub struct Point {
  point_id: usize,
  cluster_id: Option<u32>,
  values: Vec<f64>,
}

pub struct Cluster {
  pub cluster_id: u32,
  pub coord: Vec<f64>,
  pub points: Vec<Rc<Point>>
}

pub struct KMeansRunner {
  cfg: KmeansConfig,
  points: Vec<Rc<Point>>,
  clusters: Vec<Cluster>
}

impl From<Vec<u32>> for KmeansConfig {
  fn from(v: Vec<u32>) -> KmeansConfig {
    KmeansConfig{
      total_points: v[0],
      spatial_dimensions: v[1],
      k: v[2],
      max_iterations: v[3],
      has_name: false
    }
  }
}

//do I want to use a K-D tree? is that ||izable? Should I test a KD tree against my || algo?
impl KMeansRunner {
  ///set up the points. The example sets each cluster at the same spot as a random point
  pub fn new(cfg: &KmeansConfig, data: Vec<Point>) -> KMeansRunner {
    let mut points = vec_to_rc_vec(data);
    thread_rng().shuffle(&mut points);
    let clusters = points.iter_mut()
      .take(cfg.k as usize)
      .enumerate()
      .map(|(id, p)| {
        Cluster::new(id as u32, p.borrow().values.clone())
      })
      .collect::<Vec<_>>();
    KMeansRunner {
      points,
      clusters,
      cfg: cfg.clone()
    }
  }
  pub fn run(&mut self) {
    let mut iters = 0;

  }
}

impl Cluster {
  fn new(cluster_id: u32, coord: Vec<f64>) -> Cluster {
    Cluster {
      cluster_id,
      coord,
      points: Vec::new()
    }
  }
}

pub fn points_from_vec(v: Vec<f64>, dim: u32) -> Vec<Point> {
  let mut points = Vec::new();
  for (point_id, slice) in v.chunks(dim as usize).enumerate() {
    points.push(Point {
      point_id,
      cluster_id: None,
      values: slice.to_vec(),
    });
  }
  points
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn pfv_test() {
  let points = points_from_vec(vec![1.0,2.0,3.0,4.0,5.0,6.0], 3);
  assert_eq!(points.len(), 2);
  assert_eq!(points[0].values, vec![1.0, 2.0, 3.0]);
  }
}