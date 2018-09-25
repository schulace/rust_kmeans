#[derive(Debug, Clone)]
pub struct KmeansConfig {
  pub total_points: u32,
  pub spatial_dimensions: u32, //dimensions for the algorithm
  pub k: u32,
  pub max_iterations: u32,
  pub has_name: bool
}

#[derive(Debug, Clone)]
pub struct Point {
  pub point_id: usize,
  pub cluster_id: Option<u32>,
  pub values: Vec<f64>,
  pub dim: u32
}

pub struct Cluster {
  pub cluster_id: u32,
  pub coord: Vec<f64>,
  pub dim: u32
}

pub struct KMeansRunner {
  cfg: KmeansConfig,
  points: Vec<Point>,
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

impl KMeansRunner {
  pub fn new(cfg: KmeansConfig, data: Vec<Point>) -> KMeansRunner {
    //TODO come back here
  }
}

pub fn points_from_vec(v: Vec<f64>, dim: u32) -> Vec<Point> {
  let mut points = Vec::new();
  for (point_id, slice) in v.chunks(dim as usize).enumerate() {
    points.push(Point {
      point_id,
      cluster_id: None,
      values: slice.to_vec(),
      dim
    });
  }
  points
}