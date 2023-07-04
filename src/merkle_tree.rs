use sha3::{Sha3_256, Digest};
use std::ops::Rem;

pub struct MerkleTree {
  hashes: Vec<Vec<u8>>,
  count: usize
}


impl MerkleTree {
  pub fn new(elements: &[String]) -> MerkleTree {
    let leaves = Self::hash_elements(elements);
    let hashes = Self::build_hashes(leaves);
    MerkleTree{hashes, count: elements.len()}
  }

  pub fn root(&self) -> &[u8] {
    &self.hashes[0]
  }

  pub fn add(&mut self, elements: &[String]) {
    let new_leaves = Self::hash_elements(elements);
    // range (self.count - 1, ..) only tree leaves
    let old_leaves = self.hashes[{self.count - 1}..].to_vec();
    let leaves = [old_leaves, new_leaves].concat();
    self.count = leaves.len();
    self.hashes = Self::build_hashes(leaves);
  }

  pub fn get_hashes(&mut self) -> Vec<Vec<u8>> {
    self.hashes.clone()
  }

  fn hash_elements(elements: &[String]) -> Vec<Vec<u8>> {
    elements.iter().map(|e| hash(e.to_string())).collect()

  }

  // This fun creates the hierarchy of hashes and stops on the root hash 
  fn build_hashes(hashes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    if hashes.len() == 1 {
      return hashes;
    }

    let h: Vec<Vec<u8>> = hashes.chunks(2).clone().into_iter().map(|e| hash_pair(e[0].clone(), e[1].clone())).collect();
    // Each element of the result array is a node in merkle tree
    [Self::build_hashes(h), hashes].concat()
  }

  fn proof(&self, mut index: usize) -> Vec<Vec<u8>> {
    let mut proof = vec![];
    let mut i = self.count-1;

    while i != 0 {
        let h: Vec<u8> = match index.rem(2) {
            0 => self.hashes[index+1+i].clone(),
            _ => self.hashes[index+i-1].clone()
        };
        proof.append(&mut vec![h]);
        index = index/2;
        i = (i+1)/2 - 1;
    }
    proof
  }

  fn verify(&self, proof: &[Vec<u8>], mut index: usize) -> bool {
    // hash of element to verufy 
    let mut hash = self.hashes[self.count - 1 + index].clone();
    // iterating each of elements that create the proof (brother and aunts)
    for p in proof {
      // depending if left or right leaf 
      // creating parent hash 
      hash = match index.rem(2) {
        0 => hash_pair(hash, p.to_vec()),
        _ => hash_pair(p.to_vec(), hash)
      };
      // we go to the left  
      index = index / 2;
    }
    hash == self.root()
  }
}



pub fn hash(element: String) -> Vec<u8> {
  let mut hasher = Sha3_256::default();
  hasher.update(element);
  hasher.finalize().to_vec()
}

pub fn hash_pair(e1: Vec<u8>, e2: Vec<u8>) -> Vec<u8> {
  let mut hasher = Sha3_256::default();
  hasher.update([e1, e2].concat());
  hasher.finalize().to_vec()
}


#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::merkle_tree::*;

    #[test]
    fn root_hash_of_hola_moikka_is_correct() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string()]);

        assert_eq!(tree.root(), hex!("d703ed960de71d89e617a637f87813b9da95461f30d5d5030329b979ff931032"));
    }

    #[test]
    fn when_adding_two_more_elements_to_the_tree_the_root_hash_is_correct() {
        let mut tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string()]);
        tree.add( &["heippa".to_string(), "ahoj".to_string()]);
        assert_eq!(tree.root(), hex!("8321751cd2de3135bcc3ee9ad978061b284d1ec23f83279192ebcc3666c9e5cc"));
    }

    #[test]
    fn proof_for_the_first_element_of_four() {
        let tree = MerkleTree::new(&["hola".to_string(), "moikka".to_string(), "heippa".to_string(), "ahoj".to_string()]);
        let moikka = hash("moikka".to_string());
        let heippa = hash("heippa".to_string());
        let ahoj = hash("ahoj".to_string());
        let heippa_ahoj = hash_pair(heippa, ahoj);
        let expected_proof = vec![moikka, heippa_ahoj];
        assert_eq!(tree.proof(0), expected_proof);
    }

    #[test]
    fn proof_for_the_second_element_of_four() {
      let tree: MerkleTree = MerkleTree::new(&["hola".to_string(), "moikka".to_string(), "heippa".to_string(), "ahoj".to_string()]);
      let hola = hash("hola".to_string());
      let heippa = hash("heippa".to_string());
      let ahoj = hash("ahoj".to_string());
      let heippa_ahoj = hash_pair(heippa, ahoj);
      let expected_proof = vec![hola, heippa_ahoj];
      assert_eq!(tree.proof(1), expected_proof)
    }

    #[test]
    fn verify_the_fourth_element_of_eight() {
        let elements = [
            "hola".to_string(), 
            "moikka".to_string(), 
            "heippa".to_string(), 
            "ahoj".to_string(),
            "privet".to_string(), 
            "bonjour".to_string(), 
            "konichiwa".to_string(),
            "rytsas".to_string()
        ];
        let tree = MerkleTree::new(&elements);
        let proof = tree.proof(3);
        assert!(tree.verify(&proof, 3));
    }
}