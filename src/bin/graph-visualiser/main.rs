/// Sample application that reads a .g2o file from disk and visualises it using kiss3d
extern crate slam_rs;
extern crate kiss3d;
extern crate nalgebra as na;

use std::str;
use std::fmt;
use slam_rs::{Graph, GraphTrait};
use kiss3d::window::Window;
use kiss3d::light::Light;

type Color = na::Point3<f32>;
struct GraphDrawConfiguration {
    pub vertex_size: f32,
    pub vertex_color: Color,
    pub draw_vertices: bool,
    pub edge_color: Color,
    pub edge_size: f32,
    pub draw_edges: bool,
    pub root_node_color: Color,
    pub draw_root_node: bool,
}
impl Default for GraphDrawConfiguration {
    fn default() -> GraphDrawConfiguration {
        GraphDrawConfiguration {
            vertex_size: 5.0,
            vertex_color: Color::new(0.6, 1.0, 1.0),
            draw_vertices: true,
            edge_size: 5.0,
            edge_color: Color::new(1.0, 1.0, 0.0),
            draw_edges: true,
            root_node_color: Color::new(0.0, 0.0, 0.0),
            draw_root_node: true,
        }
    }
}

/// Draw a Graph instance in the given KISS 3D window
fn draw_graph<T>(graph: &Graph<T>, win: &mut Window, draw_config_: Option<GraphDrawConfiguration>) -> Result<(), &'static str>
where T: na::Real + str::FromStr,
      <T as str::FromStr>::Err: fmt::Debug,
{

    // read the config
    let draw_config = match draw_config_ {
        None => GraphDrawConfiguration::default(),
        Some(c) => c
    };

    win.set_point_size(draw_config.vertex_size);

    // iterate through the nodes position estimates, plot them
    for point in graph.get_node_pos_estimates() {
        win.draw_point(&point, &draw_config.vertex_color)
    }

    // connect the nodes
    // TODO

    Ok(())
}

fn main() {
    // graphics initialisation
    let mut window = Window::new("Graph Visualisation Example");
    window.set_light(Light::StickToCamera);
    window.set_background_color(1.0, 1.0, 1.0);

    // initialise a graph
    let g2o_fpath = "/home/berger/playground/rust/slam/slam-rs/share/slam-rs/sample-graphs/torus3D.g2o";
    let mut graph: Graph<f64> = Graph::new();
    graph.load_from_g2o(g2o_fpath).unwrap_or_else(|err| {
        eprintln!("Failed to initialise graph from file: {}\n\tError: {}", g2o_fpath, err);
    });

    while window.render() {
        draw_graph(&graph, &mut window, None).unwrap_or_else(|err| {
            eprintln!("Failed to draw the graph\n\tError: {}", err);
        });
    }
}
