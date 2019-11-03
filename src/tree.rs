#[derive(Debug)]
pub struct Tree {
    color: &'static str,
}

impl Tree {
    pub fn new(color: Option<&'static str>) -> Self {
        match color {
            Some(c) => Tree { color: c },
            None => Default::default(),
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Tree { color: "brown" }
    }
}
