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

    pub fn add(&mut self, name: String) -> &mut Node {
        let n = Node::new(name);
        self.child.push(n);
        self.child.last_mut().unwrap()
    }

    pub fn println(&self) {
        println!(".");
        self._println(false, &mut Vec::new())
    }

    fn _println(&self, is_last: bool, indent: &mut Vec<bool>) {
        for _ in indent.iter() {
            print!("{}", if is_last {INDENT_LAST_TOKEN} else {INDENT_DEFAULT_TOKEN});    
        }
        
        println!("{}{}", if is_last {LAST_ITEM_TOKEN} else {DEFAULT_TOKEN}, self.name);
        indent.push(is_last);
        let l = self.child.len();
        for i in 0..l {
            self.child[i]._println(i == l-1, indent)
        }
    }
}