use std::collections::HashMap;

pub struct TrieNode {
    value: char,
    is_final: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(c: char, is_final: bool) -> TrieNode {
        return TrieNode {
            value: c,
            is_final: is_final,
            children: HashMap::<char, TrieNode>::new(),
        };
    }

    fn check_value(&self, c: char) -> bool {
        self.print_value();
        return self.value == c;
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Box<Trie> {
        return Box::new(Trie {
            root: TrieNode::new('R', false),
        });
    }

    pub fn insert_word(&mut self, s: &String) {
        let char_str: Vec<char> = s.chars().collect();
        self.insert(&char_str);
    }

    pub fn check_word(&self, s: &String) -> bool {
        let char_str: Vec<char> = s.chars().collect();
        return self.check(&char_str);
    }

    fn insert(&mut self, char_str: &Vec<char>) {
        let mut idx: usize = 0;
        let size: usize = char_str.len();

        let mut children = &mut self.root.children;
        while idx < size {
            let is_final: bool = if idx == size-1 { true } else { false };
            let next_node = children.entry(char_str[idx])
                .or_insert(TrieNode::new(char_str[idx], is_final));
            children = &mut next_node.children;
            idx += 1;
        }
    }

    fn check(&self, char_str: &Vec<char>) -> bool {
        let mut idx: usize = 0;
        let size: usize = char_str.len();

        let mut children = &self.root.children;
        while idx < size {
            let next_node: &TrieNode;
            let try_node = children.get(&char_str[idx]);

            match try_node {
                Some(node) => next_node = node,
                None       => return false,
            }

            children = &next_node.children;
            idx += 1;
        }
        return true;
    }
}

