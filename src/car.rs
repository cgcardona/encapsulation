#[derive(Debug)]
pub struct Car {
    color: &'static str,
}

impl Car {
    pub fn new(color: Option<&'static str>) -> Self {
        match color {
            Some(c) => Car { color: c },
            None => {
                let c: Car = Default::default();
                c
            }
        }
    }
}

impl Default for Car {
    fn default() -> Self {
        Car { color: "gold" }
    }
}
