use crate::css;

#[derive(Debug, PartialEq, Default)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}
#[derive(Debug, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    Number(f32),
    ColorValue(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Px,
    Em,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Specificity = (usize, usize, usize);

pub const DEFAULT_STYLE: &str = include_str!("default.css");

impl Stylesheet {
    pub fn default_style() -> Self {
        css::parse(DEFAULT_STYLE.into())
    }

    pub fn merge(&mut self, other: Stylesheet) {
        self.rules.extend(other.rules);
    }
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            Value::Length(f, Unit::Em) => f * 16.0,
            Value::Number(f) => f, // TODO
            _ => 0.0,
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
