# Introspect Query

[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/iq.svg
[l1]: https://crates.io/crates/iq

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/iq/badge.svg
[l3]: https://docs.rs/iq/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3768?rust


IQ (Introspect Query) lets you query standard structs, maps, enums, arrays, tuples, and
nested combinations of these, to get deep values with a simple path syntax.

Values jut have to implement serde's `Serialize` trait.
Both values and queries are dynamic, and can be provided at runtime.

IQ is efficient: the explored value isn't serialized, the `Serialize` trait is used to visit it and the visit goes only to the desired target, skipping other branches.

See the [IQ](trait.IQ.html) trait for all extract functions.

```rust
use iq::IQ;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize)]
struct Car {
    pub engine: String,
    pub passengers: Vec<Dog>,
    pub driver: Dog,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Dog {
    pub name: String,
    pub ears: u8,
}

let car = Car {
    engine: "V8".to_string(),
    passengers: vec![
        Dog {
            name: "Roverandom".to_string(),
            ears: 1,
        },
        Dog {
            name: "Laïka".to_string(),
            ears: 2,
        },
    ],
    driver: Dog {
        name: "Rex".to_string(),
        ears: 2,
    },
};

// extract "primitive" values as strings with extract_primitive
assert_eq!(car.extract_primitive("driver.ears").unwrap(), "2");
assert_eq!(car.extract_primitive("driver.name").unwrap(), "Rex");
assert_eq!(car.extract_primitive("passengers.1.name").unwrap(), "Laïka");
assert_eq!(car.extract_primitive("passengers.1"), None); // it's not a primitive

// extract any value as Json with extract_json
assert_eq!(car.extract_json("wrong.path"), None);
assert_eq!(car.extract_json("driver.ears").unwrap(), "2");
assert_eq!(car.extract_json("driver.name").unwrap(), r#""Rex""#);
assert_eq!(
    car.extract_json("passengers.0").unwrap(),
    r#"{"name":"Roverandom","ears":1}"#
);
assert_eq!(car.extract_json("passengers.3"), None);

// extract any deserializable value with extract_value
assert_eq!(car.extract_value("driver.ears").unwrap(), Some(2));
assert_eq!(
    car.extract_value("passengers.1").unwrap(),
    Some(Dog {
        name: "Laïka".to_string(),
        ears: 2
    }),
);

// You don't have to concat tokens if you build the path
assert_eq!(
    car.extract_primitive(vec!["passengers", "0", "ears"])
        .unwrap(),
    "1"
);

// Extract functions are available both on the IQ trait and as standalone functions.
assert_eq!(iq::extract_primitive(&car, "driver.name").unwrap(), "Rex");

// You can extract the size of the deep array/tuple/map/struct/string
assert_eq!(car.extract_size("passengers"), Some(2)); // count of array items
assert_eq!(car.extract_size("passengers.1.name"), Some(5)); // count of chars in "Laïka"
assert_eq!(car.extract_size("passengers.1"), Some(2)); // count of fields in the Dog struct
assert_eq!(car.extract_size("passengers.3"), None); // There's no third passenger
assert_eq!(car.extract_size(""), Some(3)); // count of fields in the Car struct
assert_eq!(iq::size_of(&car), Some(3)); // same than previous, when you don't want to dive
assert_eq!(iq::size_of(&("a", 1)), Some(2)); // works with tuples too

// If iq is compiled with the "template" feature, you get a mini templating utility
let template = iq::Template::new("{driver.name} drives a {engine} car.");
assert_eq!(template.render(&car), "Rex drives a V8 car.");

```

IQ also works with enums, maps, and tuples: more tests can be found in libs.rs.

