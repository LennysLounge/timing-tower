use uuid::{uuid, Uuid};

pub enum Node {
    Directory(Directory),
    File(File),
}

pub struct Directory {
    pub id: Uuid,
    pub name: String,
    pub nodes: Vec<Node>,
}

impl Directory {
    pub fn new(name: &str, nodes: Vec<Node>) -> Node {
        Node::Directory(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            nodes,
        })
    }
}

pub struct File {
    pub id: Uuid,
    pub name: String,
}

impl File {
    pub fn new(name: &str) -> Node {
        Node::File(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
        })
    }
    pub fn new_with_id(name: &str, id: Uuid) -> Node {
        Node::File(Self {
            id,
            name: name.to_string(),
        })
    }
}

pub fn make_tree() -> Node {
    Directory::new(
        "Root",
        vec![
            Directory::new(
                "Things",
                vec![
                    Directory::new("Other things", vec![File::new("GGGG")]),
                    File::new("CCCC"),
                    File::new("DDDD"),
                ],
            ),
            File::new("AAAA"),
            File::new_with_id("ABAB", uuid!("5ef68c19-45fd-4d34-84b5-89948df109f9")),
            File::new("BBBB"),
            Directory::new("Dodads", vec![File::new("EEEE"), File::new("FFFF")]),
        ],
    )
}
