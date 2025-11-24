# api-guidelines

[![Crates.io](https://img.shields.io/crates/v/api-guidelines.svg)](https://crates.io/crates/api-guidelines)
[![Documentation](https://docs.rs/api-guidelines/badge.svg)](https://docs.rs/api-guidelines)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/slightmeta/api-guidelines/ci.yml)](https://github.com/slightmeta/api-guidelines/actions)

A comprehensive Rust library providing structured enums and utilities for the official Rust API Guidelines. This crate helps developers follow Rust best practices by providing programmatic access to API design principles.

## Overview

This library implements the complete Rust API Guidelines as structured enums, making it easier to:
- Reference specific guidelines in code
- Build linting tools and code analyzers
- Document API design decisions
- Ensure code quality and consistency
- Create educational materials and examples
- Generate API documentation with guideline references

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
api-guidelines = "0.1.0"
```

## Quick Start

```rust
use api_guidelines::{Naming, Interoperability, Predictability, Flexibility};

fn main() {
    // Reference naming conventions
    let naming_convention = Naming::C_CASE;
    let conversion_guideline = Naming::C_CONV;

    // Reference interoperability guidelines
    let common_traits = Interoperability::C_COMMON_TRAITS;
    let conversion_traits = Interoperability::C_CONV_TRAITS;

    // Reference predictability guidelines
    let smart_ptr_guideline = Predictability::C_SMART_PTR;
    let constructor_guideline = Predictability::C_CTOR;

    // Use in documentation or tooling
    println!("Following guideline: {:?}", naming_convention);
    println!("Following guideline: {:?}", conversion_traits);
    println!("Following guideline: {:?}", smart_ptr_guideline);
}
```

## Available Guidelines

### Naming Conventions
- `C_CASE` - Casing conventions (UpperCamelCase vs snake_case)
- `C_CONV` - Conversion method naming (as_, to_, into_)
- `C_GETTER` - Getter naming conventions
- `C_ITER` - Iterator method naming
- `C_ITER_TY` - Iterator type naming
- `C_FEATURE` - Feature naming conventions
- `C_WORD_ORDER` - Consistent word order

### Interoperability
- `C_COMMON_TRAITS` - Common trait implementations
- `C_CONV_TRAITS` - Standard conversion traits
- `C_COLLECT` - FromIterator and Extend implementations
- `C_SERDE` - Serde serialization support
- `C_SEND_SYNC` - Send and Sync implementations
- `C_GOOD_ERR` - Error type best practices
- `C_NUM_FMT` - Number formatting traits
- `C_RW_VALUE` - Reader/Writer function parameters

### Predictability
- `C_SMART_PTR` - Smart pointers don't add inherent methods
- `C_CONV_SPECIFIC` - Conversions live on the most specific type
- `C_METHOD` - Functions with clear receivers are methods
- `C_NO_OUT` - Functions don't take out-parameters
- `C_OVERLOAD` - Operator overloads are unsurprising
- `C_DEREF` - Only smart pointers implement Deref and DerefMut
- `C_CTOR` - Constructors are static, inherent methods

### Flexibility
- `C_INTERMEDIATE` - Functions expose intermediate results
- `C_CALLER_CONTROL` - Caller decides where to copy and place data
- `C_GENERIC` - Functions minimize assumptions using generics
- `C_OBJECT` - Traits are object-safe if useful as trait objects

### Type Safety
- `C_NEWTYPE` - Newtypes provide static distinctions
- `C_CUSTOM_TYPE` - Arguments convey meaning through types
- `C_BITFLAG` - Types for flags are bitflags, not enums
- `C_BUILDER` - Builders enable construction of complex values

### Dependability
- `C_VALIDATE` - Functions validate their arguments
- `C_DTOR_FAIL` - Destructors never fail
- `C_DTOR_BLOCK` - Destructors that may block have alternatives

### Debuggability
- `C_DEBUG` - All public types implement Debug
- `C_DEBUG_NONEMPTY` - Debug representation is never empty

### Future Proofing
- `C_SEALED` - Sealed traits protect against downstream implementations
- `C_STRUCT_PRIVATE` - Structs have private fields
- `C_NEWTYPE_HIDE` - Newtypes encapsulate implementation details
- `C_STRUCT_BOUNDS` - Data structures don't duplicate derived trait bounds

### Necessities
- `C_STABLE` - Public dependencies of stable crates are stable
- `C_PERMISSIVE` - Crate and dependencies have permissive license

### Documentation
- `C_CRATE_DOC` - Crate level docs are thorough with examples
- `C_EXAMPLE` - All items have rustdoc examples
- `C_QUESTION_MARK` - Examples use ?, not try!, not unwrap
- `C_FAILURE` - Function docs include error, panic, and safety considerations
- `C_LINK` - Prose contains hyperlinks to relevant things
- `C_METADATA` - Cargo.toml includes all common metadata
- `C_RELNOTES` - Release notes document all significant changes
- `C_HIDDEN` - Rustdoc doesn't show unhelpful implementation details

### Macros
- `C_EVOCATIVE` - Input syntax is evocative of the output
- `C_MACRO_ATTR` - Item macros compose well with attributes
- `C_ANYWHERE` - Item macros work anywhere items are allowed
- `C_MACRO_VIS` - Item macros support visibility specifiers
- `C_MACRO_TY` - Type fragments are flexible

## Documentation

Full documentation is available at [docs.rs/api-guidelines](https://docs.rs/api-guidelines).

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Based on the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
