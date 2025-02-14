use mimc_rs::Mimc7;
use num_bigint::{BigInt, Sign};
use ark_bls12_381::Fr; 
use ark_ff::PrimeField; 

#[derive(Debug)]
struct SMerkleTree {
    root: Node,
}

#[derive(Debug)]
struct Node {
    hash: String,
    value: BigInt,
}

impl SMerkleTree {
    fn construct(nodes: Vec<Node>) -> SMerkleTree {
        println!("construct merkle tree");
        SMerkleTree {
            root: Node::new("root".to_string(), BigInt::from(0))
        }
    }
    fn add_node(&self, node: Node) {
        println!("add node{:?}", node);
    }
    fn get_root(&self) -> &Node {
        &self.root
    }
    fn get_proof(&self, node: Node) -> Vec<String> {
        println!("get proof{:?}", node);
        vec!["proof".to_string()]
    }
    fn combine_nodes(&self, node1: Node, node2: Node) -> Node {
        println!("combine nodes{:?}{:?}", node1, node2);
        Node::new("combine".to_string(), BigInt::from(0))
    }
    
}

impl Node {
    fn new(hash: String, value: BigInt) -> Node {
        Node {
            hash,
            value
        }
    }
    fn hash(&self) -> String {
        let hasher = Mimc7::new();
        let mut array: Vec<BigInt> = Vec::new();
        let h = BigInt::parse_bytes(self.hash.as_bytes(), 16).unwrap();
        array.push(h);
        array.push(self.value.clone());
        hasher.hash(array);
        "mimc_hash".to_string()
    }
    fn get_hash(&self) -> &String {
        &self.hash
    }
    fn get_value(&self) -> BigInt {
        self.value.clone()
    }
    
}