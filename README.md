A map like data structure based on Vec to represent a Graph with directed edges.

The goal is to be able to build a structure looking more like this:
```
     |
  \  |  /
 \ \ | / /
__\_\|/_/__
  / /|\ \
 / / | \ \
  /  |  \
     |
```

Instead of a tree like this:
```
  ____|____
__|__   __|__
|   |   |   |
```

Further goals:
 - [x] store and find node
 - [x] store edges
 - [] graph traversal along the edges
 - [] generator function to exhaustively discover all paths from one node to another
 - [] possible weights on edges
