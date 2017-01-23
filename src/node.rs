use super::get_match_len;

#[derive(Clone, Debug, PartialEq)]
pub struct TrieNode<T> {
    /// The key associated with this node.
    key: Vec<u8>,
    /// The value associated with this node, if any.
    value: Option<T>,
    /// All branches from this node
    children: Vec<Box<TrieNode<T>>>,
}

impl<T> TrieNode<T> {
    /// Creates a new Trie
    pub fn new() -> Self {
        TrieNode {
            key: Vec::new(),
            value: None,
            children: Vec::new(),
        }
    }

    /// Retrieve the value associated with `key`. If the key is not found, `None`
    /// is returned.
    pub fn get(&self, key: &[u8]) -> Option<&T> {
        let key = key.as_ref();
        // Get the length of the match between this node's key and the given
        // `key`. If the `match_len` is the length of this node's key, we look
        // further for a match. Otherwise there is no match in this node so we
        // return `None`.
        let match_len = get_match_len(&self.key, key);
        if match_len == self.key.len() {
            // If the match length is also the length of the key we are
            // searching for, we have complete match and we return the value
            // at this node.
            // NOTE: The value with this node may still be `None`.
            if match_len == key.len() {
                return self.value.as_ref();
            } else {
                // Part of `key` still needs to be matched, so we search the
                // children of this node for a match.
                let key = &key[match_len..];
                return self.get_children(key);
            }
        }
        None
    }

    fn get_children(&self, key: &[u8]) -> Option<&T> {
        // Search all of this node's children for a matching prefix.
        for child in &self.children {
            let value = child.get(key);
            if value.is_some() {
                return value;
            }
        }

        // No match found
        None
    }

    /// Returns true if this node's key and the given `key` share a common
    /// prefix.
    pub fn prefix_match(&self, key: &[u8]) -> bool {
        get_match_len(&self.key, key) > 0
    }

    /// Inserts a key-value pair into the trie.
    pub fn insert(&mut self, key: Vec<u8>, value: T) {
        // Empty tree, simple set key/value for this node to given key/value.
        if self.key.is_empty() {
            self.key = key;
            self.value = Some(value);

            // Non-empty trie cases
        } else {
            // Get the length of the match for our nodes
            // NOTE: The length of the match should always be
            // at least 1. If the trie is not empty we have already
            // checked that there is some match.
            let match_len = get_match_len(&self.key, &key);
            debug_assert!(match_len > 0);

            // If the length of the match is the length of this node's key,
            // we do not need to split the node.
            if match_len == self.key.len() {
                let key = key[match_len..].to_vec();
                // This failing implies that we were given two of the same key.
                debug_assert!(!key.is_empty());

                // If there are no children, we just add a new node. No need to
                // worry about another node with a matching prefix.
                if self.children.is_empty() {
                    self.add_new_child(key, Some(value));
                } else {
                    self.insert_children(key, value);
                }
            } else {
                // Match length was less than the length of this node's key.
                // Split this node into two seperate nodes.
                let child_key = self.key[match_len..].to_vec();
                self.key = self.key[0..match_len].to_vec();
                let child_value = self.value.take();
                self.add_new_child(child_key, child_value);

                // Insert new node
                let key = key[match_len..].to_vec();
                // This failing implies that we were given two of the same key
                debug_assert!(!key.is_empty());

                self.add_new_child(key, Some(value));
            }
        }
    }

    /// Create a new child node with the given key-value pair and insert it
    /// as a child of this node.
    fn add_new_child(&mut self, key: Vec<u8>, value: Option<T>) {
        let child = TrieNode {
            key: key,
            value: value,
            children: Vec::new(),
        };
        self.children.push(Box::new(child));
    }

    /// Insert this key-value pair into the children of this node.
    fn insert_children(&mut self, key: Vec<u8>, value: T) {
        // Check all children of this node for one that has a
        // common prefix of any length. If a common prefix is
        // found, we insert at that node.
        for mut child in &mut self.children {
            if child.prefix_match(&key) {
                child.insert(key, value);
                return;
            }
        }

        // No matching node found, add a new child.
        self.add_new_child(key, Some(value));
    }

    /// Deletes the node matching `key` from the trie. If
    /// `key` does not represent a complete node, i.e. a node
    /// with a value, nothing happens.
    pub fn delete(&mut self, key: &[u8]) {}
}

#[cfg(test)]
mod test {
    use super::TrieNode;

    #[test]
    fn single_insert() {
        let data = "Data";
        let mut trie = TrieNode::new();
        trie.insert(b"data".to_vec(), data);

        let trie2 = TrieNode {
            key: b"data".to_vec(),
            value: Some(data),
            children: Vec::new(),
        };

        assert_eq!(trie, trie2);
        assert_eq!(trie.get(b"data"), Some(&data));
    }

    #[test]
    fn multiple_insert() {
        let mut trie = TrieNode::new();
        trie.insert(b"/".to_vec(), "data");
        trie.insert(b"/2".to_vec(), "data2");

        let trie2 = TrieNode {
            key: b"/".to_vec(),
            value: Some("data"),
            children: vec![Box::new(TrieNode {
                               key: b"2".to_vec(),
                               value: Some("data2"),
                               children: Vec::new(),
                           })],
        };

        assert_eq!(trie, trie2);
        assert_eq!(trie.get(b"/"), Some(&"data"));
        assert_eq!(trie.get(b"/2"), Some(&"data2"));
    }

    #[test]
    fn split_node() {
        let mut trie = TrieNode::new();
        trie.insert(b"/1".to_vec(), "Data");
        trie.insert(b"/2".to_vec(), "Data2");

        let trie2 = TrieNode {
            key: b"/".to_vec(),
            value: None,
            children: vec![Box::new(TrieNode {
                               key: b"1".to_vec(),
                               value: Some("Data"),
                               children: Vec::new(),
                           }),
                           Box::new(TrieNode {
                               key: b"2".to_vec(),
                               value: Some("Data2"),
                               children: Vec::new(),
                           })],
        };

        assert_eq!(trie, trie2);
        assert_eq!(trie.get(b"/"), None);
        assert_eq!(trie.get(b"/1"), Some(&"Data"));
        assert_eq!(trie.get(b"/2"), Some(&"Data2"));
    }
}
