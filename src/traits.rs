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

    fn add_node_label(&mut self, v: T, label: L);

    fn delete_edge(&mut self, v: T, w: T); // should eventually be changed to return a Result type

    fn delete_vertex(&mut self, v: T); // should eventually be changed to return a Result type

    fn e_count(&self) -> T;

    fn edit_label(&mut self, v: T);

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
