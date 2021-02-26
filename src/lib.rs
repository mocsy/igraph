use std::{collections::BTreeMap, iter::FusedIterator};

/// A node in the graph is identified by the key.
/// Keys are stored in the order they were inserted, a redundant copy is stored in the index.
/// Values don't have this redundancy.
/// There could be more than one values for a key.
#[derive(Debug, Clone)]
pub struct IndexedGraph<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    edges: BTreeMap<K, K>,
    i: BTreeMap<K, Vec<usize>>,
    // phantom: PhantomData<&'a V>,
}

impl<K: Ord + Clone, V> IndexedGraph<K, V> {
    /// Makes a new, empty `IndexedGraph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// let mut graph = BTreeMap::<u8,u8>::new();
    /// assert_eq!(core::mem::size_of::<BTreeMap<u8,u8>>(), 24);
    /// assert_eq!(core::mem::size_of_val(&graph), 24);
    ///
    /// use super_tree::IndexedGraph;
    /// let mut graph = IndexedGraph::new();
    ///
    /// assert_eq!(core::mem::size_of::<IndexedGraph<u8,u8>>(), 96);
    /// assert_eq!(core::mem::size_of_val(&graph), 96);
    ///
    /// // entries can now be inserted into the empty graph
    /// graph.insert(1, "a");
    /// ```
    pub fn new() -> IndexedGraph<K, V> {
        IndexedGraph {
            keys: vec![],
            values: vec![],
            edges: BTreeMap::new(),
            i: BTreeMap::new(),
            // phantom: PhantomData,
        }
    }

    /// Clears the graph, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "a");
    /// graph.clear();
    /// // assert!(graph.is_empty());
    /// ```
    pub fn clear(&mut self) {
        // Let's just drop everything.
        *self = IndexedGraph::new();
    }

    /// Returns a reference to the values corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "a");
    /// assert_eq!(graph.get(&1), vec![&"a"]);
    /// assert_eq!(graph.get(&2), Vec::<&&str>::new());
    /// ```
    pub fn get(&self, key: &K) -> Vec<&V> {
        let mut res = vec![];
        if let Some(indexes) = self.i.get(key) {
            for idx in indexes {
                res.push(&self.values[*idx]);
            }
        }
        return res;
    }

    /// Returns the key-value pairs corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "a");
    /// assert_eq!(graph.get_key_values(&1), vec![(&1, &"a")]);
    /// assert_eq!(graph.get_key_values(&2), vec![]);
    /// ```
    pub fn get_key_values(&self, key: &K) -> Vec<(&K, &V)> {
        let mut res = vec![];
        if let Some(tup) = self.i.get_key_value(key) {
            for idx in tup.1 {
                res.push((tup.0, &self.values[*idx]));
            }
        }
        return res;
    }

    /// Returns the first key-value pair in the graph.
    /// The key in this pair is the minimum key in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// assert_eq!(graph.first_key_value(), None);
    /// graph.insert(1, "b");
    /// graph.insert(2, "a");
    /// assert_eq!(graph.first_key_value(), Some((&1, &"b")));
    /// ```
    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        if let Some(key) = self.keys.first() {
            Some((key, &self.values[0]))
        } else {
            None
        }
    }

    /// Removes and returns the first element in the graph.
    /// The key of this element is the key first inserted into the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "a");
    /// graph.insert(2, "b");
    /// while let Some((key, _val)) = graph.pop_first() {
    ///     assert!(graph.iter().all(|(k, _v)| *k > key));
    /// }
    /// for item in graph.iter() {
    ///     assert!(*item.0 < 3);
    ///     assert!(*item.1 == "a" || *item.1 == "b");
    /// }
    /// assert!(graph.is_empty());
    /// ```
    pub fn pop_first(&mut self) -> Option<(K, V)> {
        if self.keys.is_empty() {
            None
        } else {
            let key = self.keys.remove(0);
            let value = self.values.remove(0);
            self.i.remove(&key);
            Some((key, value))
        }
    }

    /// Returns the last key-value pair in the graph.
    /// The key in this pair is last inserted in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "b");
    /// graph.insert(2, "a");
    /// assert_eq!(graph.last_key_value(), Some((&2, &"a")));
    /// ```
    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        if let Some(key) = self.keys.last() {
            Some((key, &self.values.last().unwrap()))
        } else {
            None
        }
    }

    /// Removes and returns the last element in the graph.
    /// The key of this element is the last inserted in the graph.
    ///
    /// # Examples
    ///
    /// Draining elements in descending order, while keeping a usable graph each iteration.
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "a");
    /// graph.insert(2, "b");
    /// while let Some((key, _val)) = graph.pop_last() {
    ///     assert!(graph.iter().all(|(k, _v)| *k < key));
    /// }
    /// assert!(graph.is_empty());
    /// ```
    pub fn pop_last(&mut self) -> Option<(K, V)> {
        if self.keys.is_empty() {
            None
        } else {
            let key = self.keys.pop().unwrap();
            let value = self.values.pop().unwrap();
            self.i.remove(&key);
            Some((key, value))
        }
    }

    /// Returns `true` if the graph contains a value for the specified key using the internal index.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(1, "a");
    /// assert_eq!(graph.contains_key(&1), true);
    /// assert_eq!(graph.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.i.get(key).is_some()
    }

    /// Inserts a key-value pair into the graph.
    ///
    /// If the graph did not have this key present, `None` is returned.
    ///
    /// If the graph did have this key present, the value is inserted after the existing one.
    /// Then the new value is returned.
    /// The key is not updated, only inserted the first time.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// assert_eq!(graph.insert(37, "a"), Some(&"a"));
    /// assert_eq!(graph.is_empty(), false);
    ///
    /// graph.insert(37, "b");
    /// assert_eq!(graph.insert(37, "c"), Some(&"c"));
    /// //assert_eq!(graph[&37], "c");
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<&V> {
        if let Some(indexes) = self.i.get_mut(&key) {
            indexes.push(self.values.len());
        } else {
            self.i.insert(key.clone(), vec![self.values.len()]);
        }
        self.values.push(value);
        self.keys.push(key);
        return self.values.last();
    }

    /// Inserts a key-value pair into the graph.
    ///
    /// If the graph did not have this key present, `None` is returned.
    ///
    /// If the graph did have this key present, the value is inserted after the existing one.
    /// Then the new value is returned.
    /// The key is not updated, only inserted the first time.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// assert_eq!(graph.insert(37, "a"), Some(&"a"));
    /// assert_eq!(graph.insert(12, "b"), Some(&"b"));
    /// assert_eq!(graph.is_empty(), false);
    ///
    /// graph.insert_edge(12, 37);
    /// assert_eq!(graph.insert(37, "c"), Some(&"c"));
    /// //assert_eq!(graph[&37], "c");
    /// ```
    pub fn insert_edge(&mut self, from: K, to: K) -> Option<(&K, &K)> {
        self.edges.insert(from.clone(), to);
        self.edges.get_key_value(&from)
    }

    /// Returns the number of elements in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut a = IndexedGraph::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a");
    /// assert_eq!(a.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.i.len()
    }

    /// Returns `true` if the graph contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut a = IndexedGraph::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a");
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.i.len() == 0
    }

    pub fn index_copy(&self) -> BTreeMap<K, Vec<usize>> {
        self.i.clone().into()
    }

    /// Returns a Vec of references to the values corresponding to the supplied key.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut a = IndexedGraph::new();
    /// a.insert(1, "a");
    /// assert_eq!(*a.index(1), [&"a"]);
    /// ```
    #[inline]
    pub fn index(&self, key: K) -> Vec<&V> {
        self.get(&key)
    }

    /// Gets an iterator over the entries of the graph, sorted by key.
    /// `IndexedGraph` preserves the order of insertion for `iter()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use super_tree::IndexedGraph;
    ///
    /// let mut graph = IndexedGraph::new();
    /// graph.insert(3, "c");
    /// graph.insert(2, "b");
    /// graph.insert(1, "a");
    ///
    /// for (key, value) in graph.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    ///
    /// let (first_key, first_value) = graph.iter().next().unwrap();
    /// assert_eq!((*first_key, *first_value), (3, "c"));
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            graph: &self,
            length: self.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'a, K: 'a, V: 'a> {
    graph: &'a IndexedGraph<K, V>,
    length: usize,
}

impl<'a, K: Ord + Clone, V> IntoIterator for &'a IndexedGraph<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K: 'a + Ord + Clone, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            let idx = &self.graph.len() - 1 - self.length;
            Some((&self.graph.keys[idx], &self.graph.values[idx]))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }

    fn last(self) -> Option<(&'a K, &'a V)> {
        self.graph.last_key_value()
    }

    fn min(mut self) -> Option<(&'a K, &'a V)> {
        self.next()
    }

    fn max(self) -> Option<(&'a K, &'a V)> {
        self.last()
    }
}

impl<K: Ord + Clone, V> FusedIterator for Iter<'_, K, V> {}

impl<'a, K: 'a + Ord + Clone, V: 'a> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            let idx = self.length;
            Some((&self.graph.keys[idx], &self.graph.values[idx]))
        }
    }
}

impl<K: Ord + Clone, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.length
    }
}

// impl<'a, K: Ord+Clone, V> Index<K> for &'a IndexedGraph<K, V> {
//     type Output = [&'a V];

//     #[inline]
//     fn index(&self, index: K) -> &[&'a V] {
//         self.get(&index).as_slice()
//     }
// }
