/*
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src
│   ├── main.rs
│   └── node.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── rstree-7606b252a0b14877
        │   └── rstree-7606b252a0b14877.d
        ├── examples
        ├── incremental
        │   └── rstree-305oidwy6y7vi
        │       ├── s-gta5zehn80-rh0smg-9tlsfp5vm7pjiz25z60vnx81s
        │       │   ├── 14tt7wbk1ct0aq6f.o
        │       │   └── work-products.bin
        │       └── s-gta5zehn80-rh0smg.lock
        ├── rstree
        └── rstree.d
*/

mod node;

fn create_tree() -> node::Node {
    let mut tree = node::Node::new("test_1".to_string());
    tree.add("test_2".to_string());
    tree
}

fn main() {
    let tree = create_tree();
    tree.println()
}
