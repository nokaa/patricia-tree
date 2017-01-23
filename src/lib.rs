mod node;

use node::TrieNode;

#[derive(Clone, Debug, PartialEq)]
pub struct Trie<T> {
    children: Vec<TrieNode<T>>,
}

impl<T> Trie<T> {
    /// Creates a new Trie
    pub fn new() -> Self {
        Trie { children: Vec::new() }
    }

    /// Retrieves the value associated with `key` from the trie, if any.
    pub fn get(&self, key: &str) -> Option<&T> {
        // Search every node for a match.
        for node in &self.children {
            let value = node.get(key);
            if value.is_some() {
                return value;
            }
        }

        // No matching node found
        None
    }

    /// Inserts the given key-value pair into the trie.
    pub fn insert<S: Into<String>>(&mut self, key: S, value: T) {
        let key = key.into();

        // Empty trie
        if self.children.is_empty() {
            let mut trie_node = TrieNode::new();
            trie_node.insert(key, value);
            self.children.push(trie_node);

            // Non-empty trie
        } else {
            // Check for a matching prefix in existing nodes
            for mut node in &mut self.children {
                if node.prefix_match(&key) {
                    node.insert(key, value);
                    return;
                }
            }

            // No matching prefix found, add new node
            let mut trie_node = TrieNode::new();
            trie_node.insert(key, value);
            self.children.push(trie_node);
        }
    }

    /// Deletes the node matching `key` from the trie. If
    /// `key` does not represent a complete node, i.e. a node
    /// with a value, nothing happens.
    pub fn delete(&mut self, _key: &str) {}
}

/// Returns the length of the common prefix shared between two strings.
fn get_match_len(a: &str, b: &str) -> usize {
    let mut match_len = 0;
    for (ac, bc) in a.chars().zip(b.chars()) {
        if ac == bc {
            match_len += 1;
        } else {
            break;
        }
    }
    match_len
}

#[cfg(test)]
mod test {
    use super::Trie;
    use super::node::TrieNode;

    #[test]
    fn single_insert() {
        let data = "Data";
        let mut trie = Trie::new();
        trie.insert("data", data);

        let trie2 = Trie {
            children: vec![TrieNode {
                               key: "data".to_string(),
                               value: Some(data),
                               children: Vec::new(),
                           }],
        };

        assert_eq!(trie, trie2);
        assert_eq!(trie.get("data"), Some(&data));
    }

    #[test]
    fn multiple_insert() {
        let mut trie = Trie::new();
        trie.insert("/", "data");
        trie.insert("/2", "data2");

        let trie2 = Trie {
            children: vec![TrieNode {
                               key: "/".to_string(),
                               value: Some("data"),
                               children: vec![Box::new(TrieNode {
                                                  key: "2".to_string(),
                                                  value: Some("data2"),
                                                  children: Vec::new(),
                                              })],
                           }],
        };

        assert_eq!(trie, trie2);
        assert_eq!(trie.get("/"), Some(&"data"));
        assert_eq!(trie.get("/2"), Some(&"data2"));
    }

    #[test]
    fn split_node() {
        let mut trie = Trie::new();
        trie.insert("/1", "Data");
        trie.insert("/2", "Data2");

        let trie2 = Trie {
            children: vec![TrieNode {
                               key: "/".to_string(),
                               value: None,
                               children: vec![Box::new(TrieNode {
                                                  key: "1".to_string(),
                                                  value: Some("Data"),
                                                  children: Vec::new(),
                                              }),
                                              Box::new(TrieNode {
                                                  key: "2".to_string(),
                                                  value: Some("Data2"),
                                                  children: Vec::new(),
                                              })],
                           }],
        };

        assert_eq!(trie, trie2);
        assert_eq!(trie.get("/"), None);
        assert_eq!(trie.get("/1"), Some(&"Data"));
        assert_eq!(trie.get("/2"), Some(&"Data2"));
    }
}
