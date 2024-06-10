use num::{ToPrimitive, Unsigned};

pub trait WTGraph<T>
where
    T: Unsigned + ToPrimitive,
{
    fn commit_changes();
}

pub trait Graph<T, L>
where
    T: Unsigned + ToPrimitive,
{
    fn add_edge(&mut self, v: T, w: T);

    fn add_vertex(&mut self, v: T);

    fn add_vertex_label(&mut self, v: T, label: L);

    fn delete_edge(&mut self, v: T, w: T); // should eventually be changed to return a Result type

    fn delete_vertex(&mut self, v: T); // should eventually be changed to return a Result type

    fn vertex_deleted(&self, v: T) -> bool; // true if last item in uncommitted edits for v is Edit::DeleteSelf

    fn e_count(&self) -> T;

    fn edit_label(&mut self, v: T, change: L);

    fn get_label(&self, v: T) -> Option<&L>;

    fn v_count(&self) -> T;
}

pub trait Directed<T>
where
    T: Unsigned + ToPrimitive,
{
    fn outgoing_edges(&self, vertex: T) -> Vec<T>; // should probably be changed to return an iterator instead

    fn incoming_edges(&self, vertex: T) -> Vec<T>; // likewise here
}

pub trait Undirected<T>
where
    T: Unsigned + ToPrimitive,
{
    fn edges(&self, vertex: T) -> Vec<T>;
}

pub trait Weighted<T>
where
    T: Unsigned + ToPrimitive,
{
    fn weight_of_edge(&self, from: T, to: T) -> f64;
}

pub trait GraphSearch<T>
where
    T: Unsigned + ToPrimitive,
{
    fn shortest_path_bfs(&self, from: T, to: T) -> Vec<T>; // returns the shortest path from `from` to `to` using breadth first search

    fn shortest_path_dfs(&self, from: T, to: T) -> Vec<T>; // returns the shortest path from `from` to `to` using depth first search

    fn shortest_paths(&self) -> Vec<Vec<T>>; // shortest paths from all vertices to all other vertices

    fn connected_components(&self) -> Vec<Vec<T>>; // returns all groups of vertices that are connected

    fn connected(&self, a: T, b: T) -> bool; // is a connected to b?
}
