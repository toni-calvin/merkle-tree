use merkle_tree::MerkleTree;

pub mod merkle_tree;

fn main() {
   let elements = &["Cat".to_string(), "Dog".to_string(), "Spider".to_string(), "Snake".to_string()]; 
   let mut tree = MerkleTree::new(elements);
   println!("{:x?}", tree.get_hashes());
 
}
