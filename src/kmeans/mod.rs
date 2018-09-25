extern crate rand;
use std::ops::AddAssign;
use std::cell::RefCell;
use std::fmt;
use std::time::Instant;
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
  pub coord: Vec<f64>,
  pub points: RefCell<Vec<Point>>
}


pub struct KMeansRunner {
  cfg: KmeansConfig,
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
    //below feels a bit inefficient, but idk how to do it without the copy bc of the borrow on clusters
    let closest_clusters = points.into_iter().map(|pt| (pt.closest_cluster(&slf.clusters), pt)).collect::<Vec<_>>();
    for (cluster_id, point) in closest_clusters {
      slf.clusters[cluster_id as usize] += point;
    }
    slf.clusters.iter_mut().for_each(Cluster::update_center);
    slf
  }
  //clusters keep track of their points. 
  pub fn run(&mut self) {
    println!("config for run: {:?}", self.cfg);
    let mut iters = 0;
    let mut converged = false;
    let mark_start = Instant::now();
    while iters < self.cfg.max_iterations && !converged {
      converged = true;
      let mut moving_points = Vec::new();
      for cluster in &self.clusters {
        moving_points.push(cluster.partition_unwanted(&self.clusters));
      }
      moving_points.into_iter().flat_map(Vec::into_iter) //flattening our moving points
        //then moving them into the appropriate spot
        .for_each(|(cluster_id, point)| {
          self.clusters[cluster_id as usize] += point;
          converged = false; //so long as even 1 point moves we haven't converged
        });
      self.clusters.iter_mut().for_each(|cl| cl.update_center());
      iters += 1;
    }
    let mark_end = Instant::now();
    println!("ran for {} iterations, completed in {:?}", iters, mark_end - mark_start);
    println!("clusters:");
    for cl in &self.clusters {
      println!("{}", cl);
    }
    
  }
}

impl Cluster {
  fn new(cluster_id: u32, coord: Vec<f64>) -> Cluster {
    Cluster {
      cluster_id,
      coord,
      points: RefCell::new(Vec::new())
    }
  }
  fn update_center(&mut self) {
    for (idx, i) in self.coord.iter_mut().enumerate() { //would be a good spot for SIMD to do a bunch of vector adds
      *i = 0.0;
      for p in self.points.borrow().iter() {
        *i += p.coord[idx];
      }
    *i = *i / self.points.borrow().len() as f64
    }
  }
  ///note that this will actually MUTABLY borrow and change self.points
  fn partition_unwanted(&self, clusters: &Vec<Cluster>) -> Vec<(u32, Point)> {
    let mut ret: Vec<(u32, Point)> = Vec::new();
    let mut points = self.points.borrow_mut();
    let mut i = 0;
    let mut top = points.len();
    while i < top { //the lengths I'll go to avoid a copy
      let best_cluster = points[i].closest_cluster(clusters);
      if best_cluster != self.cluster_id {
        ret.push((best_cluster, points.remove(i)));
        top -= 1;
      } else {
        i += 1
      }
    }
    ret
  }
}

impl Point {
  fn distance(&self, c: &Cluster) -> f64 {
    let ret = self.coord.iter()
      .zip(c.coord.iter())
      .map(|(a,b)| (a - b).powi(2))
      .sum::<f64>()
      .sqrt();
    ret
  }
  fn closest_cluster(&self, clusters: &Vec<Cluster>) -> u32 {
    clusters.iter()
      .map(|cl| (self.distance(cl), cl.cluster_id))
      .min_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).expect("tried a NaN comparison"))
      .unwrap().1
  }
}

impl AddAssign<Point> for Cluster {
  fn add_assign(&mut self, p: Point) {
    self.points.borrow_mut().push(p);
  }
}

impl fmt::Display for Cluster {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "cluster {{id:{}, points:{}, coord:{:?}}}", self.cluster_id, self.points.borrow().len(), self.coord)
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
    assert_eq!(cl.points.borrow().len(), 2);
    cl.update_center();
    assert_eq!(cl.coord, vec![5.0, 5.0, 5.0]);
  }

  #[test]
  fn test_closest_cluster() {
    let mut c1 = Cluster::new(0, vec![1.0, 1.0]);
    let c2 = Cluster::new(1, vec![4.0, 4.0]);
    c1 += Point{point_id: 0, coord: vec![5.0, 5.0]};
    c1 += Point{point_id: 1, coord: vec![1.0, 0.0]};
    let clusters = vec![c1, c2];
    assert_eq!(clusters[0].points.borrow()[0].closest_cluster(&clusters), 1);
  }

  #[test]
  fn test_cluster_unwanted() {
    let mut c1 = Cluster::new(0, vec![1.0, 1.0]);
    let c2 = Cluster::new(1, vec![4.0, 4.0]);
    c1 += Point{point_id: 0, coord: vec![5.0, 5.0]};
    c1 += Point{point_id: 1, coord: vec![1.0, 0.0]};
    let clusters = vec![c1, c2];
    let rejected_p = clusters[0].partition_unwanted(&clusters);
    assert_eq!(clusters[0].points.borrow().len(), 1);
    assert_eq!(rejected_p[0].1.point_id, 0);
    assert_eq!(rejected_p[0].0, 1);
    //how do we allow self-mutation while borrowing collection with self?
  }
}