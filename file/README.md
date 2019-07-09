# histo-graph-file

## Storage of objects
Each object is stored in a separate file. The object is serialized using serde and bincode (a random choice). The files are named by the hex-representation of the SHA256 hash of their content. It is important to build the hash based on the serialized object, and not on the memory representation of the object, because the memory representation might be platform-specifc.

The object types are: vertices, edges, graphs, graph-history.

## Storage of vertices
A vertex consists of a vertex_id and attributes, where the attributes are a map of key-value pairs. Each vertex is stored as an object. If the vertex changes over time (i.e. its attributes change), the SHA256 hash of the serialized vertex will change, and the different versions of the vertex will be stored in different files. Note that on storage level, the vertex does not contain any information about outgoing or incoming edges from and to itself, respectively.
Vertices are stored in the sub-directory `vertex/` of the storage directory.

## Storage of edges
At storage level, an edge consists of three pieces of information: the SHA256 hash of the vertex that the edge goes out from, the hash of the vertex that the edge comes in to, and the attributes of the edge. When a vertex changes, its SHA256 hash changes, and therefore the edges connected to that vertex change as well. New versions of the edges connected to the vertex have to be stored.

## Storage of a graph
At storage level, a graph consist of an ordered list of SHA256 hashes of vertices, and and ordered list of hashes of edges. This data is serialized and is itself stored as an object.

### Storage of the set of vertices that belong to a graph
The set of vertices that belong to a graph is itself stored as an object.
These sets are stored in the sub-directory `vertexvec` of the storage directory.

## Storage of the history of a graph
