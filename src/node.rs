static DEFAULT_TOKEN: &str = "├──";
static LAST_ITEM_TOKEN: &str = "└──";
static INDENT_DEFAULT_TOKEN: &str = "│   ";
static INDENT_LAST_TOKEN: &str = "    ";

#[derive(Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: Node::new("".to_string()),
        }
    }

    pub fn println(&self) {
        println!(".");
        let l = self.root.child.len();
        for i in 0..l {
            self.root.child[i].println(i == l - 1, &Vec::new())
        }
    }

    pub fn add(&mut self, name: String) -> &mut Node {
        self.root.add(name)
    }
}

#[derive(Debug)]
pub struct Node {
    name: String,
    child: Vec<Node>,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            name: name,
            child: Vec::new(),
        }
    }

    pub fn add(&mut self, name: String) -> &mut Node {
        let n = Node::new(name);
        self.child.push(n);
        self.child.last_mut().unwrap()
    }

    fn println(&self, is_last: bool, indent: &[bool]) {
        for last in indent.iter() {
            print!(
                "{}",
                if *last {
                    INDENT_LAST_TOKEN
                } else {
                    INDENT_DEFAULT_TOKEN
                }
            );
        }
        println!(
            "{}{}",
            if is_last {
                LAST_ITEM_TOKEN
            } else {
                DEFAULT_TOKEN
            },
            self.name
        );

        let l = self.child.len();
        let mut vec = indent.to_vec();
        vec.push(is_last);
        for i in 0..l {
            self.child[i].println(i == l - 1, &vec)
        }
    }
}
