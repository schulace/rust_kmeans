
extern crate rand;
use std::ops::AddAssign;
use kmeans::rand::prelude::*; //this feels really weird

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
  coord: Vec<f64>
}

#[derive(Clone, Debug)]
pub struct Cluster {
  pub cluster_id: u32,
  pub coord: Vec<f64>, //gonna get copied, but not TOO eggregious since * of copies is K*D*Iters
  pub points: Vec<Point>
}

#[derive(Clone, Debug)]
pub struct NoPointsCluster {
  pub cluster_id: u32,
  pub coord: Vec<f64>,
}

pub struct KMeansRunner {
  cfg: KmeansConfig,
  clusters: Vec<Cluster>
}

impl<'a> From<&'a Cluster> for NoPointsCluster {
  fn from(cl: &Cluster) -> NoPointsCluster {
    NoPointsCluster {
      cluster_id: cl.cluster_id,
      coord: cl.coord.clone()
    }
  }
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
  pub fn new(cfg: &KmeansConfig, mut points: Vec<Point>) -> KMeansRunner {
    thread_rng().shuffle(&mut points); //randomize the order of the points
    let clusters = points.iter()
      .take(cfg.k as usize)
      .enumerate()
      .map(|(id, p)| {
        Cluster::new(id as u32, p.coord.clone())
      })
      .collect::<Vec<_>>();
    let mut slf = KMeansRunner {
      clusters,
      cfg: cfg.clone()
    };
    unimplemented!() //place all points 
  }
  //clusters keep track of their points. 
  pub fn run(&mut self) {
    let mut iters = 1;

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
  fn update_center(&mut self) {
    for (idx, i) in self.coord.iter_mut().enumerate() { //would be a good spot for SIMD to do a bunch of vector adds
      *i = 0.0;
      for p in &mut self.points {
        *i += p.coord[idx];
      }
    *i = *i / self.points.len() as f64
    }
  }
  fn partition_unwanted(&mut self, clusters: &Vec<NoPointsCluster>) -> Vec<(u32, Point)> {
    let mut ret: Vec<(u32, Point)> = Vec::new();
    let mut i = 0;
    let mut top = self.points.len();
    while i < top { //the lengths I'll go to avoid a copy
      let best_cluster = self.points[i].closest_cluster(clusters);
      if best_cluster != self.cluster_id {
        ret.push((best_cluster, self.points.remove(i)));
        top -= 1;
      } else {
        i += 1
      }
    }
    ret
  }
}

impl Point {
  fn distance(&self, c: &NoPointsCluster) -> f64 {
    let ret = self.coord.iter()
      .zip(c.coord.iter())
      .map(|(a,b)| (a - b).powi(2))
      .sum::<f64>()
      .sqrt();
    ret
  }
  fn closest_cluster(&self, clusters: &Vec<NoPointsCluster>) -> u32 {
    clusters.iter()
      .map(|cl| (self.distance(cl), cl.cluster_id))
      .min_by(|(d1, c1), (d2, c2)| d1.partial_cmp(d2).expect("tried a NaN comparison"))
      .unwrap().1
  }
}

impl AddAssign<Point> for Cluster {
  fn add_assign(&mut self, p: Point) {
    self.points.push(p);
  }
}

pub fn points_from_vec(v: Vec<f64>, dim: u32) -> Vec<Point> {
  let mut points = Vec::new();
  for (point_id, slice) in v.chunks(dim as usize).enumerate() {
    points.push(Point {
      point_id,
      coord: slice.to_vec()
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
  assert_eq!(points[0].coord, vec![1.0, 2.0, 3.0]);
  }

  #[test]
  fn test_cluster_add_and_update() {
    let mut cl = Cluster::new(0, vec![1.0,2.0,3.0]);
    cl += Point{point_id: 0, coord: vec![0.0, 0.0, 0.0]};
    cl += Point{point_id: 1, coord: vec![10.0, 10.0, 10.0]};
    assert_eq!(cl.points.len(), 2);
    cl.update_center();
    assert_eq!(cl.coord, vec![5.0, 5.0, 5.0]);
  }

  #[test]
  fn test_closest_cluster() {
    let mut c1 = Cluster::new(0, vec![1.0, 1.0]);
    let c2 = Cluster::new(1, vec![4.0, 4.0]);
    c1 += Point{point_id: 0, coord: vec![5.0, 5.0]};
    c1 += Point{point_id: 1, coord: vec![1.0, 0.0]};
    let mut clusters = vec![c1, c2];
    let npClusters = clusters.iter().map(NoPointsCluster::from).collect::<Vec<_>>();
    assert_eq!(clusters[0].points[0].closest_cluster(&npClusters), 1);
  }

  #[test]
  fn test_cluster_unwanted() {
    let mut c1 = Cluster::new(0, vec![1.0, 1.0]);
    let c2 = Cluster::new(1, vec![4.0, 4.0]);
    c1 += Point{point_id: 0, coord: vec![5.0, 5.0]};
    c1 += Point{point_id: 1, coord: vec![1.0, 0.0]};
    let mut clusters = vec![c1, c2];
    let np_clusters = clusters.iter().map(NoPointsCluster::from).collect::<Vec<_>>();
    let rejected_p = clusters[0].partition_unwanted(&np_clusters);
    assert_eq!(clusters[0].points.len(), 1);
    assert_eq!(rejected_p[0].1.point_id, 0);
    assert_eq!(rejected_p[0].0, 1);
    //how do we allow self-mutation while borrowing collection with self?
  }
}