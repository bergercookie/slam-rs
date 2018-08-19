pub mod graphs;

pub use self::graphs::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_from_g2o() {
        // TODO - Make this relevant to the root of the package
        let g2o_fpath = "/home/berger/playground/rust/slam/slam-rs/share/slam-rs/sample-graphs/torus3D.g2o";
        let g2o_num_nodes = 5000;
        let g2o_num_edges = 9048;
        // Move the Graph creation code to some common infrastructure
        let mut g: Graph<f64> = Graph::new();

        // should be empty at first
        assert_eq!(g.get_nodes_count(), 0);
        assert_eq!(g.get_factors_count(), 0);

        // Fill the factors from the g2o file
        let res = g.load_from_g2o(g2o_fpath);
        assert_eq!(res, Ok(()));

        // assert that there's the expected number of data in the graph
        assert_eq!(g.get_nodes_count(), g2o_num_nodes);
        assert_eq!(g.get_factors_count(), g2o_num_edges);
    }

    #[test]
    fn TODO_graph_add_factor_wrong_dimensions() {}

    #[test]
    #[should_panic(expected="No such file or directory")]
    fn load_from_g2o_invalid_filepath() {
        let g2o_fpath = "a_surely_invalid_path";
        let mut g: Graph<f64> = Graph::new();
        g.load_from_g2o(g2o_fpath);
    }

    #[test]
    fn graph_test_get_root() {
        let g2o_fpath = "/home/berger/playground/rust/slam/slam-rs/share/slam-rs/sample-graphs/torus3D.g2o";
        let mut g: Graph<f64> = Graph::new();
        assert_eq!(g.get_root(), None);

        // TODO - Make this relevant to the root of the package
        let res = g.load_from_g2o(g2o_fpath);
        assert_eq!(res, Ok(()));
        assert_eq!(g.get_root(), Some(0))
    }
}
