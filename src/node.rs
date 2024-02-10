#[warn(dead_code)]
static DEFAULT_TOKEN:        &str = "├──";
static LAST_ITEM_TOKEN:      &str = "└──";
static INDENT_DEFAULT_TOKEN: &str = "│   ";
static INDENT_LAST_TOKEN:    &str = "    ";


pub struct Node {
    name:  String,
    child: Vec<Node>,
}

impl Node {
    pub fn new(name: String) -> Self {
        let c = Vec::new();
        Node {
            name: name,
            child: c
        }
    }

    pub fn add(&mut self, name: String) {
        let n = Node::new(name);
        self.child.push(n)
    }

    pub fn println(&self) {
        println!(".");
        self._println(false)
    }

    fn _println(&self, is_last: bool) {
        let l = self.child.len();
        println!("{}{}", if is_last {LAST_ITEM_TOKEN} else {DEFAULT_TOKEN}, self.name);

        for i in 0..l {
            self.child[i]._println(i == l-1)
        }
    }
}