
** WORK IN PROGRESS - Not in working state **

This is a very basic attempt at implementing a SLAM algorithm in rust. It should
include:

- Parsing of graphs from g2o files
- Graph construction (PoseGraphs, Pose / Landmarks etc.)
- Basic graph optimisation via DogLeg / Levenberg Marquardt
- Graph visualization via [kiss3d](https://github.com/sebcrozet/kiss3d)

