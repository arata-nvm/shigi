use std::collections::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Document {
    pub root_node: Node,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub children: Vec<Node>,
    pub typ: NodeType,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        typ: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        typ: NodeType::Element(ElementData {
            tag_name: name,
            attrs,
        }),
    }
}

impl Document {
    pub fn new(root_node: Node) -> Self {
        Self { root_node }
    }

    fn collect_tags<'a>(&self, node: &'a Node, tag_name: &str, nodes: &mut Vec<&'a ElementData>) {
        if let NodeType::Element(ref node) = node.typ {
            if node.tag_name == tag_name {
                nodes.push(node);
            }
        }

        for child in &node.children {
            self.collect_tags(child, tag_name, nodes);
        }
    }

    pub fn collect_css_pathes(&self) -> Vec<String> {
        let mut links = Vec::new();
        self.collect_tags(&self.root_node, "link", &mut links);

        links
            .iter()
            .filter(|node| {
                node.attrs
                    .get("rel")
                    .map_or(false, |rel| rel == "stylesheet")
            })
            .filter_map(|node| node.attrs.get("href").cloned())
            .collect()
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attrs.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attrs.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}
