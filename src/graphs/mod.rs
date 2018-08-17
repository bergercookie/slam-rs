extern crate nalgebra as na;

use base::types::NodeId;
use factors::FactorTrait;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};

pub trait GraphTrait {
    /// get the list of nodes registered so far
    fn get_node_ids(&self) -> Vec<NodeId>;
    /// Get the number of nodes
    fn get_nodes_count(&self) -> usize {
        self.get_node_ids().len()
    }

    fn get_factors(&self) -> Vec<Box<FactorTrait>>;
    /// Get the number of factors
    fn get_factors_count(&self) -> usize {
        self.get_factors().len()
    }

    /// Initialise a Graph from a textfile in the g2o format
    ///
    /// Format is explained here: https://github.com/RainerKuemmerle/g2o/wiki/File-Format
    fn load_from_g2o(&mut self, fname: &str);
    /// Export a Graph to a textfile in the g2o format
    fn export_to_g2o(&self, fname: &str) {
        unimplemented!()
    }

    /// Initialise a graph from a textfile in the format used by TORO/MRPT
    ///
    /// * https://openslam-org.github.io/toro.html
    /// * https://www.mrpt.org/Graph-SLAM_maps
    ///
    fn load_from_toro(&mut self, _fname: &str) {
        unimplemented!()
    }
    /// Export a Graph to a textfile in the TORO/MRPT format
    fn export_to_toro(&self, _fname: &str) {
        unimplemented!()
    }

    /// Render the Graph - e.g., with kiss3d
    fn visualise_graph(&self) {}
}

pub struct Graph<F> {
    /// list of factors in the graph
    factors: Vec<Box<F>>,
}

impl<F> Graph<F> {
    pub fn new() -> Graph<F> {
        Graph {
            factors: Vec::new(),
        }
    }
}

impl<F> GraphTrait for Graph<F> {
    fn load_from_g2o(&mut self, fname: &str) {
        // read contents of file, add factors, variables as necessary
        let f = File::open(fname).unwrap();
        let f = BufReader::new(f);
        for line in f.lines() {
            let line = line.unwrap();
            let mut line_iter = line.split_whitespace();

            // handle type of edge
            let line_type = line_iter.next();
            let line_type_spl: Vec<_> = line_type.unwrap().split(':').collect();
            println!("line_type_spl.len(): {:#?}", line_type_spl.len());
            let line_type = line_type_spl[0];
            let mut angle_repr: Option<String> = None;
            if line_type_spl.len() == 2 {
                angle_repr = Some(line_type_spl[1].to_string());
            }

            // handle value of edge
            if line_type == "VERTEX_SE3" {
                continue;
            } else if line_type == "EDGE_SE3" {

                // create XYZ translation
                // TODO

                // Create quaternion
                // TODO Be able to specify other angle formats

            }
            let line_val = line_iter.next();
        }
    }

    /// TODO
    fn get_node_ids(&self) -> Vec<NodeId> {
        Vec::new()
    }
    /// TODO
    fn get_factors(&self) -> Vec<Box<FactorTrait>> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factors::BetweenPosesFactor;
    use geometry::poses::Pose3D;

    #[test]
    fn load_from_g2o() {
        // Move the Graph creation code to some common infrastructure
        let mut g: Graph<BetweenPosesFactor<Pose3D<f64>>> = Graph::new();

        // should be empty at first
        assert_eq!(g.get_nodes_count(), 0);
        assert_eq!(g.get_factors_count(), 0);

        // Fill the factors from the g2o file
        g.load_from_g2o(
            "/home/berger/playground/rust/slam/slam-rs/share/slam-rs/sample-graphs/torus3D.g2o",
        );

        // assert that there's data in the graph
        assert_eq!(g.get_nodes_count(), 1000);
        assert_eq!(g.get_factors_count(), 1000);
    }
}
