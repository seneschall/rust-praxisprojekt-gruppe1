// This module is offering functionality to instantiate complete graphs from files. The file needs to contain the graph's data in the following format:
// Note that the lines are numbered for clairity. do not include the line numbers in your wt-/graph-input-file.\
// | Line   | Tab1                                 | Tab2              | Tab3 (optional)           |
// |--------|--------------------------------------|---------------    |---------------------------|
// | (1)    | <number of vertices in the graph>    |       -           |          -                |
// | (2)    | <number of edges in the graph>       |       -           |         -                 |
// | (3)    | <vertex_from>                        | <vertex_to>       |          -                |
// | (3)    | <vertex_from>                        | <vertex_to>       | <weight_as_type_W>        |
// | (3)    | <vertex_from> as L                   | <vertex_to> as L  |          -                |
// | (3)    | <vertex_from> as L                   | <vertex_to> as L  | <weight_as_type_W>        |
// Note also that the number of lines in the file should be equal to the number of the edges that you declare+2.
// () Find example API calls in the documentation for the import functions
pub mod graphen;

pub mod wt_graphen;
