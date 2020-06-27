** WORK IN PROGRESS - Not in working state **

This is a very basic attempt at implementing a visual-based SLAM algorithm in
Rust. It should include:

* Reading images from a dataset in the [EuRoc](https://projects.asl.ethz.ch/datasets/doku.php?id=kmavvisualinertialdatasets) format.
* Compute detectors/descriptors on the images
* Construct a graph of constraints
* Solve the graph using the Gauss - Newton algorithm
* Visualise results
