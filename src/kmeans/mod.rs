extern crate rand;
mod myiters;
use self::rand::seq::sample_slice_ref;
use self::rand::Rng;
use self::myiters::SortedSliceIter;
use std::fmt;

#[derive(Debug, Clone)]
pub struct KmeansConfig {
  pub total_points: u32,
  pub spatial_dimensions: u32, //dimensions for the algorithm
  pub k: u32,
  pub max_iterations: u32,
  pub has_name: bool, //always false
}

#[derive(Debug, Clone)]
pub struct Point {
  point_id: usize,
  cluster_id: usize,
  coord: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Cluster {
  pub cluster_id: usize,
  pub coord: Vec<f64>,
}

pub struct KMeansRunner {
  pub cfg: KmeansConfig,
  pub clusters: Vec<Cluster>,
  pub points: Vec<Point>,
}

impl From<Vec<u32>> for KmeansConfig {
  fn from(v: Vec<u32>) -> KmeansConfig {
    KmeansConfig {
      total_points: v[0],
      spatial_dimensions: v[1],
      k: v[2],
      max_iterations: v[3],
      has_name: false,
    }
  }
}

//do I want to use a K-D tree? is that ||izable? Should I test a KD tree against my || algo?
impl KMeansRunner {
  ///set up the points. The example sets each cluster at the same spot as a random point
  pub fn new<R: Rng>(cfg: &KmeansConfig, points: Vec<Point>, random_src: &mut R) -> KMeansRunner {
    let clusters = sample_slice_ref(random_src, &points, cfg.k as usize)
      .iter()
      .enumerate()
      .map(|(id, p)| Cluster::new(id as u32, p.coord.clone()))
      .collect::<Vec<_>>();

    KMeansRunner {
      clusters,
      cfg: cfg.clone(),
      points,
    }
  }
  //clusters keep track of their points.
  pub fn run(&mut self) -> u32 {
    let mut iters = 0;
    let mut converged = false;
    while iters < self.cfg.max_iterations && !converged {
      converged = true;
      for point in &mut self.points {
        converged = point.change_cluster(&self.clusters) && converged;
      }
      self.points.sort_by(|p1, p2| p1.cluster_id.cmp(&p2.cluster_id));
      let grouped_points_iterator = SortedSliceIter::new(&self.points, |p1, p2| p1.cluster_id == p2.cluster_id);
      for (cluster, point_slice) in self.clusters.iter_mut().zip(grouped_points_iterator) {
        cluster.update_center(point_slice);
      }
      iters += 1;
    }
    iters
  }
  pub fn print_clusters(&self) {
    for cluster in &self.clusters {
      println!("{}", cluster);
    }
  }
}

impl Cluster {
  fn new(cluster_id: u32, coord: Vec<f64>) -> Cluster {
    Cluster {
      cluster_id: cluster_id as usize,
      coord,
    }
  }
  fn update_center(&mut self, points: &[Point]) {
    for (idx, i) in self.coord.iter_mut().enumerate() {
      //would be a good spot for SIMD to do a bunch of vector adds
      *i = 0.0;
      for p in points {
        *i += p.coord[idx];
      }
      *i = *i / points.len() as f64
    }
  }
}

impl Point {
  fn distance(&self, c: &Cluster) -> f64 {
    let ret = self
      .coord
      .iter()
      .zip(c.coord.iter())
      .map(|(a, b)| (a - b).powi(2))
      .sum::<f64>()
      .sqrt();
    ret
  }
  fn closest_cluster(&self, clusters: &Vec<Cluster>) -> usize {
    clusters
      .iter()
      .map(|cl| (self.distance(cl), cl.cluster_id))
      .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).expect("tried a NaN comparison"))
      .unwrap()
      .1
  }
  fn change_cluster(&mut self, clusters: &Vec<Cluster>) -> bool {
    let old_cluster = self.cluster_id;
    self.cluster_id = self.closest_cluster(&clusters);
    old_cluster == self.cluster_id
  }
}

impl fmt::Display for Cluster {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "cluster {{id:{}, coord:{:?}}}",
      self.cluster_id, self.coord
    )
  }
}

pub fn points_from_vec(v: Vec<f64>, dim: u32) -> Vec<Point> {
  let mut points = Vec::new();
  for (point_id, slice) in v.chunks(dim as usize).enumerate() {
    points.push(Point {
      point_id,
      cluster_id: 0, //a tad hacky
      coord: slice.to_vec(),
    });
  }
  points
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn pfv_test() {
    let points = points_from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3);
    assert_eq!(points.len(), 2);
    assert_eq!(points[0].coord, vec![1.0, 2.0, 3.0]);
  }

  #[test]
  fn test_cluster_add_and_update() {
    let mut cl = Cluster::new(0, vec![1.0, 2.0, 3.0]);
    let points = points_from_vec(vec![9.0, 8.0, 7.0, 5.0, 5.0, 5.0], 3);
    cl.update_center(points.as_slice());
    assert_eq!(cl.coord, vec![5.0, 5.0, 5.0]);
  }
}
