use std::{collections::HashMap, hash::Hash};

pub struct Graph<T>
where
    T: Eq,
    T: Clone,
{
    nodes: HashMap<T, Vec<T>>,
}

impl<T> Graph<T>
where
    T: Eq,
    T: Clone,
    T: Hash,
{
    fn get_child_nodes(&self, node: &T) -> impl Iterator<Item = T> + '_ {
        self.nodes.get(node).unwrap().iter().cloned()
    }

    fn get_shortest_path_to_child(&self, node: &T, target_node: &T) -> Vec<T> {
        let mut path = vec![];
        self.get_shortest_path_to_child_rec(node, target_node, &mut path);
        path
    }

    /// recursive fn
    fn get_shortest_path_to_child_rec(
        &self,
        node: &T,
        target_node: &T,
        path: &mut Vec<T>,
    ) -> Option<()> {
        path.push(node.clone());

        if let Some(child_nodes) = self.nodes.get(node) {
            for child_n in child_nodes {
                if child_n == target_node {
                    path.push(target_node.clone());
                    return Some(());
                }

                if let Some(_) = self.get_shortest_path_to_child_rec(child_n, target_node, path) {
                    return Some(());
                }
            }
        }
        path.pop();
        None
    }
}

#[cfg(test)]
mod graph_tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use super::Graph;
    /// test data:
    /// ```
    ///    1
    ///  2   3
    /// 4 5 6 7
    /// ```
    fn init_graph() -> Graph<usize> {
        Graph {
            nodes: HashMap::from([(1, vec![2, 3]), (2, vec![4, 5]), (3, vec![6, 7])]),
        }
    }

    #[test]
    fn get_child_nodes() {
        let graph = init_graph();
        assert_eq!(graph.get_child_nodes(&1).collect_vec(), vec![2, 3]);
        assert_eq!(graph.get_child_nodes(&3).collect_vec(), vec![6, 7]);
    }

    #[test]
    fn get_shortest_path_to_child() {
        let graph = init_graph();
        assert_eq!(graph.get_shortest_path_to_child(&1, &3), vec![1, 3]);
        assert_eq!(graph.get_shortest_path_to_child(&1, &5), vec![1, 2, 5]);
        assert_eq!(graph.get_shortest_path_to_child(&3, &7), vec![3, 7]);
    }
}
