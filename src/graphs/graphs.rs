use base::types::NodeId;
use factors::{FactorTrait, BetweenPosesFactor};
use geometry::poses::{PoseTrait, PoseN};
use std::str;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::marker::PhantomData;
use itertools::Itertools;

use rand::prelude::*;
use ::generic_array;
use ::typenum;
use ::std;
use ::na;

pub trait GraphTrait<T: na::Real, D: na::Dim> {
    /// get the list of nodes registered so far
    fn get_node_ids(&self) -> Vec<NodeId>;
    /// Get the number of nodes
    fn get_nodes_count(&self) -> usize {
        self.get_node_ids().len()
    }

    fn get_factors(&self) -> &Vec<Box<FactorTrait<D>>>;
    /// Get the number of factors
    fn get_factors_count(&self) -> usize {
        self.get_factors().len()
    }

    fn get_root(&self) -> Option<NodeId>;

    /// Add a new factor to the graph
    fn add_factor(&mut self, factor: Box<FactorTrait<D>>) -> Result<(), &'static str>;

    /// Initialise a Graph from a textfile in the g2o format
    ///
    /// Format is explained here: https://github.com/RainerKuemmerle/g2o/wiki/File-Format
    fn load_from_g2o(&mut self, fname: &str) -> Result<(), &'static str>;
    /// Export a Graph to a textfile in the g2o format
    fn export_to_g2o(&self, fname: &str) -> Result<(), &'static str> {
        unimplemented!()
    }

    /// Initialise a graph from a textfile in the format used by TORO/MRPT
    ///
    /// * https://openslam-org.github.io/toro.html
    /// * https://www.mrpt.org/Graph-SLAM_maps
    ///
    fn load_from_toro(&mut self, _fname: &str) -> Result<(), &'static str> {
        unimplemented!()
    }
    /// Export a Graph to a textfile in the TORO/MRPT format
    fn export_to_toro(&self, _fname: &str) -> Result<(), &'static str> {
        unimplemented!()
    }

    /// Render the Graph - e.g., with kiss3d
    fn visualise_graph(&self) -> Result<(), &'static str> {
        Ok(())
    }

    // TODO - Implement this
    fn dijkstra_node_estimate(&mut self) {
        unimplemented!()
    }

    /// Get an estimate of the positions of the NodeIds in space
    /// This may include both vertices or landmarks in the graph
    fn get_node_pos_estimates(&self) -> Vec<na::Point3<f32>>;
}

/// A graph of Factors
/// TODO Implement dimensions: * `D` - Dimensions of the graph (2D, 3D, etc.)
pub struct Graph<T: na::Real> {
    /// list of factors in the graph
    factors: Vec<Box<FactorTrait<na::U3>>>,
    _marker: PhantomData<T>
}

impl<T: na::Real> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph { factors: Vec::new(), _marker: PhantomData }
    }
}

impl<T> GraphTrait<T, na::U3> for Graph<T>
where T: na::Real,
      T: na::Real + str::FromStr,
      <T as str::FromStr>::Err: fmt::Debug, {

    /// TODO - Make this more efficient - maybe cache the nodes list?
    fn get_node_ids(&self) -> Vec<NodeId> {
        // TODO - You need to implement this method in the corresponding FactorTrait and then just
        // call it from here

        let mut node_ids = Vec::new();
        for f in self.factors.iter() {
            node_ids.extend(f.get_node_ids());
        }

        node_ids.into_iter().unique().collect()
    }

    fn get_factors(&self) -> &Vec<Box<FactorTrait<na::U3>>> {
        &self.factors
    }

    fn add_factor(&mut self, factor: Box<FactorTrait<na::U3>>) -> Result<(), &'static str> {
        self.factors.push(factor);
        Ok(())
    }

    /// Return the first valid NodeID that you can get from iterating through the factors list and
    /// their associated `NodeId`s
    fn get_root(&self) -> Option<NodeId> {
        let mut root_node = None;
        match self.get_nodes_count() {
            0 => {},
            _ => {
                let factors = self.get_factors();
                for f in factors.iter() {
                    println!("f.get_node_ids(): {:?}", f.get_node_ids());
                    match f.get_node_ids().as_slice() {
                        [] => { continue; } ,
                        [first, ..] => {
                            root_node = Some(first.clone());
                            break;
                        }
                    }
                }
            }
        }

        root_node
    }

    fn load_from_g2o(&mut self, fname: &str) -> Result<(), &'static str> {
        // read contents of file, add factors, variables as necessary
        let f = File::open(fname).unwrap();
        let f = BufReader::new(f);
        for line in f.lines() {
            let line = line.unwrap();
            let mut line_iter = line.split_whitespace();

            // handle type of edge
            let line_type = line_iter.next();
            let line_type_spl: Vec<_> = line_type.unwrap().split(':').collect();
            let line_type = line_type_spl[0];
            // TODO - Enable
            // let mut angle_repr: Option<String> = None;
            // if line_type_spl.len() == 2 {
            //     angle_repr = Some(line_type_spl[1].to_string());
            // }

            // Example:
            // EDGE_SE3:QUAT 4877 4977   0.100898 -0.810923 -0.475330   0.6337326 -0.4920395 -0.5958230 0.0357092   100.000000 0.000000 0.000000 0.000000 0.000000 0.000000   100.000000 0.000000 0.000000 0.000000 0.000000   100.000000 0.000000 0.000000 0.000000   400.000000 0.000000 0.000000   400.000000 0.000000   400.000000

            // TODO - check the dimensions of the edges in the file
            if line_type.starts_with("VERTEX_SE3") {
                continue;
            } else if line_type.starts_with("EDGE_SE") {

                    // handle value of edge
                    let from = line_iter.next().unwrap().parse::<NodeId>().unwrap();
                    let to = line_iter.next().unwrap().parse::<NodeId>().unwrap();
                    let pose = line_iter.collect::<Vec<_>>().join(" ");
                    let factor = Box::new(
                        BetweenPosesFactor::<T, na::U3>::new(from, to, PoseN::<T, na::U3>::from_string(pose)),
                        );

                // add it to the graph
                self.add_factor(factor).unwrap();
            }
        }

        Ok(())
    }

    // TODO -implement this
    // TODO - Return tuple of NodeId to estimated position instead
    fn get_node_pos_estimates(&self) -> Vec<na::Point3<f32>> {
        let mut rng = thread_rng();
        let mut out = vec![];

        for _ in 0..self.get_nodes_count() {
            let val1: f32 = rng.gen_range(0.0, 100.0);
            let val2: f32 = rng.gen_range(0.0, 100.0);
            let val3: f32 = rng.gen_range(0.0, 100.0);

            out.push(na::Point3::new(val1, val2, val3))
        }
        out
    }
}

