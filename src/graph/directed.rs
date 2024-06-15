// USE ?

// UNIT-TESTS for Digraph and Weighed Digraph
#[cfg(test)]
mod test {
    use super::*;
    const V_COUNT: u32 = 10;
}

// Digraph - definition and methods
pub struct Digraph<L>
{
    v_count: usize,                     // number of vertices
    e_count: usize,                     // number of edges
    adj: Vec<Vec<usize>>,               // adjacency list of indices -- note from group: should we set this to pub(crate)?
    node_labels: HashMap<L>,            // format: index of node - value of node's label
}

impl<L> Digraph<L>
{
    pub fn new(v_count: usize) -> Self {
        Digraph {
            v_count,
            e_count: 0,
            adj: vec![vec![]; v_count],
            node_labels: HashMap::new(),
        }
    }
    pub fn from_adjacency_list(v_count: usize, e_count: usize, adj: Vec<Vec<usize>>) -> Self {
        // temporary, constructor with adj list -- note from celine: what is the meaning of this comment? is there a missing implementation?
        Digraph {
            v_count,
            e_count,
            adj,
            node_labels: HashMap::new(),
        }
    }
    fn vertex_exists(&self, v: usize) -> bool {
        v < self.v_count
    }
}

impl<L> Graph<L> for Digraph<L>
where
    L: Clone,
{
    fn add_edge(&mut self, v: usize, w: usize) {
        if !(self.vertex_exists(v) || self.vertex_exists(w)) {
            panic!("One of vertices {}, {} doesn't exist", v, w)
        };
        self.e_count += self.e_count();
        self.adj[v].push(w);
    }

    fn add_vertex(&mut self, v: usize) {
        todo!() // ...
    }

    fn add_vertex_label(&mut self, v: usize, label: L) {
        self.node_labels.insert(v, label); 
    }

    fn append_vertex(&mut self) -> usize {
        todo!() // ...
    }

    fn delete_edge(&mut self, v: usize, w: usize) {
        let i_of_w: usize; // -- note from celine: could we use index_of_w for clarity?
        match self.adj.get(v) {
            Some(vs) => {  
                let i_of_w_opt = vs.iter().position(|&x| x == w); // -- note from celine: can you explain this?
                // is this a nested match?
                match i_of_w_opt {
                    Some(i) => {
                        i_of_w = i;
                    } // swap_remove more efficient than remove because the order is not important
                    None => {
                        panic!("There was no edge from {v} to {w}.");
                    }
                }
            }
            None => {
                panic!("Vertex {v} doesn't exist."); // Should be replaced by Result type
            }
        }

        self.adj[v].swap_remove(i_of_w);
        self.e_count -= self.e_count();
    }

    fn delete_vertex(&mut self, v: usize) {
        todo!() // ...
    }

    fn e_count(&self) -> usize {
        self.e_count
    }

    fn edit_label(&mut self, v: usize, change: L) { // change <-> label
        todo!() // ...
    }

    fn get_label(&self, v: usize) -> Option<&L> {
        self.node_labels.get(&v) // note from celine: can you explain this?
    }

    fn v_count(&self) -> usize {
        self.v_count
    }

    fn vertex_deleted(&self, v: usize) -> bool {
        todo!() // ...
    }
}

impl<L> Directed for Digraph<L>
    // no where L clone?
{
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        self.adj[vertex].clone()
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        todo!() // ...
    }
}
