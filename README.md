# xml-schema
[![Build Status]][travis] [![Latest Version]][crates.io] [![Coverage Status]][coveralls]


[Build Status]: https://travis-ci.org/media-io/xml-schema.svg?branch=master
[travis]: https://travis-ci.org/media-io/xml-schema
[Latest Version]: https://img.shields.io/crates/v/xml-schema.svg
[crates.io]: https://crates.io/crates/xml-schema
[Coverage Status]: https://coveralls.io/repos/github/media-io/xml-schema/badge.svg?branch=master
[coveralls]: https://coveralls.io/github/media-io/xml-schema?branch=master

Generate rust code (structures and enum) from XSD 

## Requirements

This project depends to other required libraries to start with, add them to your `Cargo.toml`.
- [YaSerDe derive](https://crates.io/crates/yaserde_derive)
- [YaSerDe](https://crates.io/crates/yaserde)
- [XML-Schema derive](https://crates.io/crates/xml-schema-derive)
- [XML-Schema](https://crates.io/crates/xml-schema)
- [XML-rs](https://crates.io/crates/xml-rs)
- [log](https://crates.io/crates/log)

## Usage

In the entrypoint of your rust project, add these folowing lines:

```rust
#[macro_use]
extern crate yaserde_derive;

use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::{YaDeserialize, YaSerialize};
```

Then implement the XSD using:

```rust
#[derive(Debug, XmlSchema)]
#[xml_schema(source = "path_to_schema.xsd", target_prefix = "my_prefix")]
struct MySchema;
```

Remark: the `MySchema` don't need to be public. It serve just as support of information.  

### Attributes

**source**: Source of the XSD - XML Schema. It can be local file (related to the root of the project) or an HTTP resource.  
**target_prefix**: The schema not define any prefix. It the `targetNamespace` is declared in the schema, this attribute is required.  
**store_generated_code**: Optional attribute for debug purpose. It store the generated Rust code into the file - the attribute value is the output filename.  
**log_level**: To configure the logger level at the the compile time - usefull if the XSD generate some bugs. Values can be `error`, `warn`, `info`, `debug`, `trace`.  
**module_namespace_mapping**: map a namespace to a Rust module. It can be present many times to map multiple namespaces to different Rust modules.  

