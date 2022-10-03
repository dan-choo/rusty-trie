mod trie;

fn main() {
    println!("Hello, world!");

    let mut trie = trie::Trie::new();
    let dog = String::from("dog");
    let doggo = String::from("doggo");
    trie.insert_word(&dog);
    trie.insert_word(&doggo);
    println!("{}", trie.check_word(&dog));
    println!("{}", trie.check_word(&doggo));
}
