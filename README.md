# one-graph
Open Source Timelined Graph Database (Work In Progress)

## Status
* At the moment OneGraph Database supports a few gremlin queries.
* Pattern Matching with a VF2 graph sub-graph isomorphism algorithm.
* Property Graph storage
* With a B+Tree for indexes
* Fixed size Records files for Nodes, Relationships and Properties.
* A fixed sized Pager implementation.

## Test
A docker package is available to test the Database:  
```
docker run -p8182:8182 --rm -it ghcr.io/alexandre-ricciardi/alexandre-ricciardi/one-graph:latest
```

This will expose a gremlin endpoint on 8182 port.

## Roadmap
* Study VF3 version of sub-graph isomorphism algorithm.
* Keep in mind that graph structures may be timelined in order to be able to retrieve past graph states.
* Improve Gremlin support.

