# KD Tree

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://badge.fury.io/gh/tterb%2FHyde.svg)](https://badge.fury.io/gh/tterb%2FHyde)

Data structure for efficiently finding points in a k-dimensional space.

This is an under development implementation of a KD Tree in Rust. 
Below is a list of features that are currently implemented and features that are planned to be implemented.

* [x] Build Tree
* [x] Find All Points Within A Radius
* [x] Find Nearest Neighbor
* [x] Insert New Point
* [x] Find **N** Nearest Neighbors
* [ ] Delete Point
* [ ] Re-Balance Tree
* [ ] Serialize Tree
* [ ] Publish Crate
* [ ] Add **K** dimensions **(Currently only 2D)**
* [x] Add Examples

This was developed initially as a way to learn Rust and to implement a KD Tree for a boids simulation although the
simulation is in progress. I plan to continue to work on this project as I learn more about Rust and as I have time.

### Performance
The performance of the KD Tree is not yet optimized. I plan to optimize the performance once I have implemented all of the features. 
The current performance was taken from `rustup run nightly cargo bench` and is as follows:

|  Size  |         Build Tree<br/>`O(n)`         | Find all points within a radius<br/>`O(n log n)` | Find nearest neighbor<br/>`O(log n)` | Insert<br/>`O(1)` |
|:------:|:-------------------------------------:|:------------------------------------------------:|:------------------------------------:|-------------------|
| 10000  |            `5,798,8 84 ns`            |                 `4,167,605 n s`                  |                                      |                   |           
| 10000  |             `0.005799 s`              |                   `0.004176 s`                   |                                      |                   |           
| 100000 |            `89,055,903 ns`            |                 `473,910,975 ns`                 |                                      |                   |           
| 100000 |              `0.05799 s`              |                    `0.4176 s`                    |                                      |                   |

## Usage - TODO
Publishing is a WIP

```rust
use kd_tree::KDTree;

fn main() {
    let mut node: KdNode<i32> = KdNode::new();
    // Tree Root
    node.insert(1, 1);
    node.insert(2, 2);
    node.insert(2, -12);

    assert_eq!(node.nearest_neighbor(Point{x: 1, y: 1}, 1.0), vec![Point{x: 1, y: 1}]);
}
```
Below is a diagram showing how the KD Tree is structured. 
The KD Tree is a binary tree where each node is a point in the k-dimensional space.
Each alternating level of the tree is split by a different dimension. 
The root node is split by the first dimension, the children of the root node are split by the second dimension,
this is typically the x and y dimensions in a 2D space. 3D space would be split by x, y, and z dimensions.

<img src="https://file.notion.so/f/s/15611bfa-6330-4686-b5db-d0509f2ad4b9/Untitled.png?id=bf8bbcc5-e466-4580-8424-747cd4874480&table=block&spaceId=2f85f1a3-8655-4f22-9bbf-94a37faa948d&expirationTimestamp=1684512561225&signature=OpplJlJj06lbknuvVPGstj9D0Tfe4qmf7cEx3VMgH2U&downloadName=Untitled.png" alt="Image" style="width:33%"><img src="https://yasenh.github.io/post/kd-tree/1.png" alt="Image" style="width:66%;">


## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## References
* [KD Tree](https://en.wikipedia.org/wiki/K-d_tree)
* [KD Tree Visualization](https://www.cs.usfca.edu/~galles/visualization/KDTree.html)
* [KD Tree Nearest Neighbor](https://www.cs.cmu.edu/~ckingsf/bioinfo-lectures/kdtrees.pdf)
* [Proof for neighborhood computation in expected logarithmic time - Martin Skrodzki](https://arxiv.org/pdf/1903.04936.pdf)
* [Introduction to a KD Tree](https://yasenh.github.io/post/kd-tree/)

## License
[MIT](LICENSE)

