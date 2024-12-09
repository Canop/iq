//! IQ (Introspect Query) lets you query standard structs, maps, enums, arrays, tuples, and
//! nested combinations of these, to get deep values with a simple path syntax.
//!
//! Values jut have to implement serde's `Serialize` trait.
//! Bot values and queries are dynamic, and can be provided at runtime.
//!
//! See the [IQ](trait.IQ.html) trait for all extract functions.
//!
//! ```rust
//! use iq::IQ;
//! use serde::{ Deserialize, Serialize };
//!
//! #[derive(Debug, Serialize)]
//! struct Car {
//!     pub engine: String,
//!     pub passengers: Vec<Dog>,
//!     pub driver: Dog,
//! }
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct Dog {
//!     pub name: String,
//!     pub ears: u8,
//! }
//!
//! let car = Car {
//!     engine: "V8".to_string(),
//!     passengers: vec![
//!         Dog {
//!             name: "Roverandom".to_string(),
//!             ears: 1,
//!         },
//!         Dog {
//!             name: "Laïka".to_string(),
//!             ears: 2,
//!         },
//!     ],
//!     driver: Dog {
//!         name: "Rex".to_string(),
//!         ears: 2,
//!     },
//! };
//!
//! // extract "primitive" values as strings with extract_primitive
//! assert_eq!(car.extract_primitive("driver.ears").unwrap(), "2");
//! assert_eq!(car.extract_primitive("driver.name").unwrap(), "Rex");
//! assert_eq!(car.extract_primitive("passengers.1.name").unwrap(), "Laïka");
//! assert_eq!(car.extract_primitive("passengers.1"), None); // it's not a primitive
//!
//! // extract any value as Json with extract_json
//! assert_eq!(car.extract_json("wrong.path"), None);
//! assert_eq!(car.extract_json("driver.ears").unwrap(), "2");
//! assert_eq!(car.extract_json("driver.name").unwrap(), r#""Rex""#);
//! assert_eq!(
//!     car.extract_json("passengers.0").unwrap(),
//!     r#"{"name":"Roverandom","ears":1}"#
//! );
//! assert_eq!(car.extract_json("passengers.3"), None);
//!
//! // extract any deserializable value with extract_value
//! assert_eq!(car.extract_value("driver.ears").unwrap(), Some(2));
//! assert_eq!(
//!     car.extract_value("passengers.1").unwrap(),
//!     Some(Dog {
//!         name: "Laïka".to_string(),
//!         ears: 2
//!     }),
//! );
//!
//! // You don't have to concat tokens if you build the path
//! assert_eq!(
//!     car.extract_primitive(vec!["passengers", "0", "ears"])
//!         .unwrap(),
//!     "1"
//! );
//!
//! // Extract functions are available both on the IQ trait and as standalone functions.
//! assert_eq!(iq::extract_primitive(&car, "driver.name").unwrap(), "Rex");
//!
//!
//! // If iq is compied with the "template" feature, you get a mini templating utility
//! let template = iq::Template::new("{driver.name} drives a {engine} car.");
//! assert_eq!(template.render(&car), "Rex drives a V8 car.");
//!
//! ```
//!
//! IQ also works with enums, maps, and tuples: more tests can be found in libs.rs.
//!

mod diver;
mod errors;
mod extract;
mod iq;
mod path;

#[cfg(feature = "template")]
mod template;

pub use {
    errors::IqError,
    extract::*,
    iq::*,
    path::*,
};

#[cfg(feature = "template")]
pub use template::*;

#[cfg(test)]
mod tests {
    use {
        super::IQ,
        serde::{
            Deserialize,
            Serialize,
        },
        std::collections::HashMap,
    };

    #[test]
    fn structs_and_arrays() {
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
        assert_eq!(car.extract_primitive("passengers.1"), None,);

        // extract any value as Json with extract_json
        assert_eq!(car.extract_json("wrong.path"), None);
        assert_eq!(car.extract_json("driver.ears").unwrap(), "2");
        assert_eq!(car.extract_json("driver.name").unwrap(), r#""Rex""#);
        assert_eq!(
            car.extract_json("passengers.0").unwrap(),
            r#"{"name":"Roverandom","ears":1}"#
        );
        assert_eq!(car.extract_json("passengers.3"), None);

        // extract any Deserializable value with extract_value
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
    }

    #[test]
    fn structs_enums_maps_and_tuples() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Dog {
            pub name: String,
            pub ears: u8,
        }
        #[derive(Debug, Serialize, PartialEq, Eq, Hash)]
        #[serde(rename_all = "snake_case")]
        enum Realm {
            Real,
            Fantasy,
        }
        #[derive(Debug, Serialize, Default)]
        struct World {
            pub targets: HashMap<String, (u32, u32, u32)>,
            pub masters: HashMap<Realm, Dog>,
        }
        let mut world = World::default();
        world.targets.insert("Earth".to_string(), (1, 2, 3));
        world.targets.insert("Moon".to_string(), (4, 5, 6));
        world.masters.insert(Realm::Fantasy, Dog {
            name: "Roverandom".to_string(),
            ears: 1,
        });
        world.masters.insert(Realm::Real, Dog {
            name: "Laïka".to_string(),
            ears: 2,
        });

        assert_eq!(world.extract_primitive("targets.Earth.1").unwrap(), "2");
        assert_eq!(world.extract_json("targets.Moon").unwrap(), "[4,5,6]");
        assert_eq!(world.extract_primitive("targets.Moon.2").unwrap(), "6");
        assert_eq!(world.extract_primitive("targets.Moon.3"), None);
        assert_eq!(
            world.extract_primitive("masters.fantasy.name").unwrap(),
            "Roverandom"
        );
        assert_eq!(world.extract_primitive("masters.real.ears").unwrap(), "2");
        assert_eq!(
            world.extract_value("masters.fantasy").unwrap(),
            Some(Dog {
                name: "Roverandom".to_string(),
                ears: 1,
            }),
        );
    }
}
