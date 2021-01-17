mod topography;
use nalgebra::Vector2;
use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use topography::Topography;
struct Lift {
    start: Vector2<u32>,
    end: Vector2<u32>,
}
struct World {
    topography: Topography,
    lifts: Vec<Lift>,
}
struct GraphRepr {
    graph: DiGraph<Vector2<i32>, u32>,
    indicies: Vec<Vec<NodeIndex<u32>>>,
}
fn grid_graph(topo: &Topography, slope_function: fn(f32) -> u32) -> GraphRepr {
    let mut graph = Graph::new();
    let indicies: Vec<Vec<NodeIndex<u32>>> = (0..topo.dimensions.x)
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
                let slope = slope_function(topo.slope(source, end));

                graph.add_edge(
                    indicies[i as usize][j as usize],
                    indicies[i as usize - 1][j as usize],
                    slope,
                );
                let slope = slope_function(topo.slope(end, source));
                graph.add_edge(
                    indicies[i as usize - 1][j as usize],
                    indicies[i as usize][j as usize],
                    slope,
                );
            }
            if j >= 1 {
                let end = Vector2::new(i, j - 1);
                let slope = slope_function(topo.slope(source, end));

                graph.add_edge(
                    indicies[i as usize][j as usize],
                    indicies[i as usize][j as usize - 1],
                    slope,
                );
                let slope = slope_function(topo.slope(end, source));
                graph.add_edge(
                    indicies[i as usize][j as usize - 1],
                    indicies[i as usize][j as usize],
                    slope,
                );
            }
        }
    }
    GraphRepr { graph, indicies }
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
impl Up {
    fn slope(x: f32) -> u32 {
        if x < 0.0 {
            1000000
        } else {
            (x * 1.0) as u32
        }
    }
}
impl DecisionTreeNode for Up {
    fn children(&self) -> Vec<Box<dyn DecisionTreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
    fn cost(&self, world: &World, position: Vector2<u32>) -> (f32, Vector2<u32>) {
        let g = grid_graph(&world.topography, Self::slope);
        let (cost, position) = world
            .lifts
            .iter()
            .map(|lift| {
                let start = g.indicies[lift.start.x as usize][lift.start.y as usize];
                let end = g.indicies[lift.end.x as usize][lift.end.y as usize];
                let path = dijkstra(&g.graph, start, Some(end), |x| 1);
                let cost: u32 = path.iter().map(|(_, cost)| cost).sum();

                (cost, lift.end)
            })
            .fold(
                (u32::MAX, Vector2::new(0, 0)),
                |(acc_cost, acc_lift), (x_cost, x_lift)| {
                    if acc_cost < x_cost {
                        (acc_cost, acc_lift)
                    } else {
                        (x_cost, x_lift)
                    }
                },
            );

        (cost as f32, position)
    }
    fn name(&self) -> String {
        "Up".to_string()
    }
}
struct Down {}
impl Down {
    fn slope(x: f32) -> u32 {
        if x > 0.0 {
            100000
        } else {
            (x.abs() * 1.0) as u32
        }
    }
}
impl DecisionTreeNode for Down {
    fn children(&self) -> Vec<Box<dyn DecisionTreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
    fn cost(&self, world: &World, position: Vector2<u32>) -> (f32, Vector2<u32>) {
        let g = grid_graph(&world.topography, Self::slope);
        let (cost, position) = world
            .lifts
            .iter()
            .map(|lift| {
                let start = g.indicies[position.x as usize][position.y as usize];
                let end = g.indicies[lift.start.x as usize][lift.start.y as usize];
                let path = dijkstra(&g.graph, start, Some(end), |x| x.weight().clone());
                let cost: u32 = path.iter().map(|(_, cost)| cost.clone().clone()).sum();

                (cost, lift.end)
            })
            .fold(
                (u32::MAX, Vector2::new(0, 0)),
                |(acc_cost, acc_lift), (x_cost, x_lift)| {
                    if acc_cost < x_cost {
                        (acc_cost, acc_lift)
                    } else {
                        (x_cost, x_lift)
                    }
                },
            );

        (cost as f32, position)
    }
    fn name(&self) -> String {
        "Down".to_string()
    }
}

fn new_world() -> World {
    World {
        topography: Topography::cone(Vector2::new(100, 100), Vector2::new(50.0, 50.0), -1.0),
        lifts: vec![Lift {
            start: Vector2::new(0, 0),
            end: Vector2::new(10, 10),
        }],
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
