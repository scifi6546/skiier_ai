mod topography;
use nalgebra::Vector2;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use topography::Topography;
struct Lift {
    start: Vector2<u32>,
    end: Vector2<u32>,
}
struct World {
    topo: Topography,
    lifts: Vec<Lift>,
}
fn grid_graph(topo: Topography) -> DiGraph<Vector2<i32>, f32> {
    let mut graph = Graph::new();
    let idx: Vec<Vec<NodeIndex<u32>>> = (0..topo.dimensions.x)
        .map(|i| {
            (0..topo.dimensions.y)
                .map(|j| graph.add_node(Vector2::new(i as i32, j as i32)))
                .collect()
        })
        .collect();
    for i in 0..topo.dimensions.x {
        for j in 0..topo.dimensions.y {
            let source = Vector2::new(i, j);
            if i >= 1 {
                let end = Vector2::new(i - 1, j);
                let slope = topo.slope(source, end);

                graph.add_edge(
                    idx[i as usize][j as usize],
                    idx[i as usize - 1][j as usize],
                    slope,
                );
                let slope = topo.slope(end, source);
                graph.add_edge(
                    idx[i as usize - 1][j as usize],
                    idx[i as usize][j as usize],
                    -1.0 * slope,
                );
            }
            if j >= 1 {
                let end = Vector2::new(i, j - 1);
                let slope = topo.slope(source, end);

                graph.add_edge(
                    idx[i as usize][j as usize],
                    idx[i as usize][j as usize - 1],
                    slope,
                );
                let slope = topo.slope(end, source);
                graph.add_edge(
                    idx[i as usize][j as usize - 1],
                    idx[i as usize][j as usize],
                    -1.0 * slope,
                );
            }
        }
    }
    graph
}

trait DecisionTreeNode {
    fn children(&self) -> Vec<Box<dyn DecisionTreeNode>>;
    fn cost(&self, world: &World, position: Vector2<u32>) -> (f32, Vector2<u32>);
    fn name(&self) -> String;
    fn best_path(
        &self,
        path_length: usize,
        world: &World,
        position: Vector2<u32>,
    ) -> Vec<(f32, String)> {
        if path_length == 0 {
            vec![(self.cost(world, position).0, self.name())]
        } else {
            let mut best = vec![];
            let mut best_weight = f32::MAX;
            for child in self.children().iter() {
                let weight = child
                    .best_path(path_length - 1, world, position)
                    .iter()
                    .fold(0.0, |acc, (weight, _)| acc + weight);
                if weight < best_weight {
                    best = vec![];
                    let (cost, position) = self.cost(world, position);
                    best.push((cost, self.name()));
                    for c in child.best_path(path_length - 1, &world, position).iter() {
                        best.push(c.clone());
                    }
                    best_weight = weight;
                }
            }
            best
        }
    }
}
struct Up {}
impl DecisionTreeNode for Up {
    fn children(&self) -> Vec<Box<dyn DecisionTreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
    fn cost(&self, world: &World, position: Vector2<u32>) -> (f32, Vector2<u32>) {
        (1.0, position)
    }
    fn name(&self) -> String {
        "Up".to_string()
    }
}
struct Down {}
impl DecisionTreeNode for Down {
    fn children(&self) -> Vec<Box<dyn DecisionTreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
    fn cost(&self, world: &World, position: Vector2<u32>) -> (f32, Vector2<u32>) {
        (10.0, position)
    }
    fn name(&self) -> String {
        "Down".to_string()
    }
}

fn new_world() -> World {
    World {
        topo: Topography::flat(Vector2::new(100, 100)),
        lifts: vec![],
    }
}
pub fn main() {
    let n: Box<dyn DecisionTreeNode> = Box::new(Up {});

    for (cost, name) in n.best_path(2, &new_world(), Vector2::new(0, 0)) {
        println!("{}, {}", cost, name);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
