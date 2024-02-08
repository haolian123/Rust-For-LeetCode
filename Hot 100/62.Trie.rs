/*
    题目：208. 实现 Trie (前缀树)
    链接：https://leetcode.cn/problems/implement-trie-prefix-tree/
 */
const ALPHABET_SIZE: usize = 26; // 假设只处理小写英文字母

struct TrieNode {
    children: [Option<Box<TrieNode>>; ALPHABET_SIZE],
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(), // 初始化所有子节点为None
            is_end_of_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            if node.children[index].is_none() {
                node.children[index] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[index].as_mut().unwrap().as_mut(); // 移动到下一个节点
        }
        node.is_end_of_word = true; // 标记单词结束
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap(); // 移动到下一个节点
        }
        node.is_end_of_word // 检查是否为一个单词的结束
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            let index = (c as usize) - ('a' as usize);
            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap(); // 移动到下一个节点
        }
        true // 找到了前缀
    }
}
