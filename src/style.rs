use crate::css::{Rule, Selector, SimpleSelector, Specificity, Stylesheet, Value};
use crate::html::{ElementData, Node, NodeType};
use std::collections::HashMap;

pub type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

#[derive(Debug)]
pub enum Display {
    Inline,
    Block,
    None,
}

#[derive(Debug)]
pub enum Position {
    Static,
    Relative,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn value_or(&self, name: &str, default: &Value) -> Value {
        self.specified_values.get(name).unwrap_or(default).clone()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }

    pub fn position(&self) -> Position {
        match self.value("position") {
            Some(Value::Keyword(s)) => match &*s {
                "static" => Position::Static,
                "relative" => Position::Relative,
                _ => Position::Static,
            },
            _ => Position::Static,
        }
    }
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    let specified_values = match root.typ {
        NodeType::Element(ref elem) => specified_values(elem, stylesheet),
        NodeType::Text(_) => HashMap::new(),
    };
    StyledNode {
        node: root,
        children: root
            .children
            .iter()
            .map(|child| child_style_tree(child, stylesheet, &specified_values))
            .collect(),
        specified_values,
    }
}

fn child_style_tree<'a>(
    root: &'a Node,
    stylesheet: &'a Stylesheet,
    parent_values: &PropertyMap,
) -> StyledNode<'a> {
    let mut values = inherited_values(parent_values);
    let specified_values = match root.typ {
        NodeType::Element(ref elem) => specified_values(elem, stylesheet),
        NodeType::Text(_) => HashMap::new(),
    };
    values.extend(specified_values);

    StyledNode {
        node: root,
        children: root
            .children
            .iter()
            .map(|child| child_style_tree(child, stylesheet, &values))
            .collect(),
        specified_values: values,
    }
}

fn inherited_values(parent_values: &PropertyMap) -> PropertyMap {
    let mut values = HashMap::new();

    let inherited_decl_names = ["font-size"];

    for decl_name in inherited_decl_names {
        if let Some(value) = parent_values.get(decl_name) {
            values.insert(decl_name.to_string(), value.clone());
        }
    }

    values
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for decl in &rule.declarations {
            match decl.name.as_str() {
                "margin" => match decl.values.len() {
                    1 => {
                        values.insert("margin".into(), decl.values[0].clone());
                    }
                    2 => {
                        values.insert("margin-top".into(), decl.values[0].clone());
                        values.insert("margin-bottom".into(), decl.values[0].clone());
                        values.insert("margin-left".into(), decl.values[1].clone());
                        values.insert("margin-right".into(), decl.values[1].clone());
                    }
                    3 => {
                        values.insert("margin-top".into(), decl.values[0].clone());
                        values.insert("margin-bottom".into(), decl.values[1].clone());
                        values.insert("margin-left".into(), decl.values[1].clone());
                        values.insert("margin-right".into(), decl.values[2].clone());
                    }
                    4 => {
                        values.insert("margin-top".into(), decl.values[0].clone());
                        values.insert("margin-bottom".into(), decl.values[1].clone());
                        values.insert("margin-left".into(), decl.values[2].clone());
                        values.insert("margin-right".into(), decl.values[3].clone());
                    }
                    _ => {}
                },
                _ => {
                    values.insert(decl.name.clone(), decl.values[0].clone());
                }
            }
        }
    }

    values
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }

    return true;
}
