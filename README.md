# Encapsulation

This is a small demo to show how I prefer to encapsulate the creation of new data structs and ensure a default implementation.

## Usage

First, clone the repo

```
git clone https://github.com/cgcardona/encapsulation.git
```

Next, change directories and build the app and deps.

```
cd encapsulation
cargo build
```

Now run the app

```
./target/debug/encapsulation
Car {
    color: "blue",
}
Car {
    color: "gold",
}
Tree {
    color: "green",
}
Tree {
    color: "brown",
}
```

## Details

Example code below from [./src/car.rs](./src/car.rs).

I like to have structs in their own separate module. I prefer to keep all the fields private and have a public `new` associated function which takes all the data needed to configure the appropriate fields.

I also like to have a `Default` implementation which `new` falls back to in the case of no data provided.

The only part I'm still unclear on is if it's idiomatic to pass `Option` in to `new` in this way.

```rs
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
```
