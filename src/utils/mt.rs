use mimc_rs::*;

#[derive(Debug)]
struct SMerkleTree {
    root: Node,
}

#[derive(Debug)]
struct Node {
    hash: String,
    value: u128,
}

impl SMerkleTree {
    fn construct(nodes: Vec<Node>) -> SMerkleTree {
        println!("construct merkle tree");
        SMerkleTree {
            root: Node::new("root".to_string(), 0)
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
        Node::new("combine".to_string(), 0)
    }
    
}

impl Node {
    fn new(hash: String, value: u128) -> Node {
        Node {
            hash,
            value
        }
    }
    fn hash(&self) -> String {
        "mimc_hash".to_string()
    }
    fn get_hash(&self) -> &String {
        &self.hash
    }
    fn get_value(&self) -> u128 {
        self.value
    }
    
}