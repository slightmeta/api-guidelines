//! # API Guidelines
//!
//! A Rust library providing structured enums and utilities for the official Rust API Guidelines.
//!
//! This crate implements the Rust API Guidelines as structured enums, making it easier to:
//! - Reference specific guidelines in code
//! - Build linting tools  
//! - Document API design decisions
//! - Ensure code quality and consistency
//!
//! ## Example
//! ```
//! use api_guidelines::{Naming, Interoperability};
//!
//! // Reference naming conventions
//! let naming_convention = Naming::C_CASE;
//! let conversion_guideline = Naming::C_CONV;
//!
//! // Reference interoperability guidelines
//! let common_traits = Interoperability::C_COMMON_TRAITS;
//! let conversion_traits = Interoperability::C_CONV_TRAITS;
//! ```
//!
//! Based on the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

#![allow(non_camel_case_types)]

/// Naming conventions and guidelines for Rust APIs
#[derive(Debug)]
pub enum Naming {
    /// In general, Rust tends to use UpperCamelCase for "type-level" constructs (types and traits) and snake_case for "value-level" constructs.
    ///
    /// In UpperCamelCase, acronyms and contractions of compound words count as one word: use Uuid rather than UUID, Usize rather than USize or Stdin rather than StdIn. In snake_case, acronyms and contractions are lower-cased: is_xid_start.
    ///
    /// [ Casing conforms to RFC 430 ](https://rust-lang.github.io/api-guidelines/naming.html)
    C_CASE,
    /// Conversions should be provided as methods, with names prefixed as follows:
    /// <table>
    ///   <thead>
    ///     <tr>
    ///       <th>Prefix</th>
    ///       <th>Cost</th>
    ///       <th>Ownership</th>
    ///     </tr>
    ///   </thead>
    ///   <tbody>
    ///     <tr>
    ///       <td><code>as_</code></td>
    ///       <td><code>Free</code></td>
    ///       <td><code>borrowed -> borrowed</code></td>
    ///     </tr>
    ///     <tr>
    ///       <td><code>to_</code></td>
    ///       <td><code>Expensive</code></td>
    ///       <td><code>borrowed -> borrowed borrowed -> owned (non-Copy types) owned -> owned (Copy types)</code></td>
    ///     </tr>
    ///     <tr>
    ///       <td><code>into_</code></td>
    ///       <td><code>Variable</code></td>
    ///       <td><code>owned -> owned (non-Copy types)</code></td>
    ///     </tr>
    ///   </tbody>
    /// </table>
    ///
    /// For example:
    ///
    /// [str::as_bytes()](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes) gives a view of a str as a slice of UTF-8 bytes, which is free. The input is a borrowed &str and the output is a borrowed &[u8].
    ///
    /// [Path::to_str](https://doc.rust-lang.org/std/path/struct.Path.html#method.to_str) performs an expensive UTF-8 check on the bytes of an operating system path. The input and output are both borrowed. It would not be correct to call this as_str because this method has nontrivial cost at runtime.
    ///
    /// [str::to_lowercase()](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase) produces the Unicode-correct lowercase equivalent of a str, which involves iterating through characters of the string and may require memory allocation. The input is a borrowed &str and the output is an owned String.
    ///
    /// [f64::to_radians()](https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians) converts a floating point quantity from degrees to radians. The input is f64. Passing a reference &f64 is not warranted because f64 is cheap to copy. Calling the function into_radians would be misleading because the input is not consumed.
    ///
    /// [String::into_bytes()](https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes) extracts the underlying Vec<u8> of a String, which is free. It takes ownership of a String and returns an owned Vec<u8>.
    ///
    /// [BufReader::into_inner()](https://doc.rust-lang.org/std/io/struct.BufReader.html#method.into_inner) takes ownership of a buffered reader and extracts out the underlying reader, which is free. Data in the buffer is discarded.
    ///
    /// [BufWriter::into_inner()](https://doc.rust-lang.org/std/io/struct.BufWriter.html#method.into_inner) takes ownership of a buffered writer and extracts out the underlying writer, which requires a potentially expensive flush of any buffered data.
    ///
    /// Conversions prefixed as_ and into_ typically decrease abstraction, either exposing a view into the underlying representation (as) or deconstructing data into its underlying representation (into). Conversions prefixed to_, on the other hand, typically stay at the same level of abstraction but do some work to change from one representation to another.
    ///
    /// When a type wraps a single value to associate it with higher-level semantics, access to the wrapped value should be provided by an into_inner() method. This applies to wrappers that provide buffering like BufReader, encoding or decoding like GzDecoder, atomic access like AtomicBool, or any similar semantics.
    ///
    /// If the mut qualifier in the name of a conversion method constitutes part of the return type, it should appear as it would appear in the type. For example Vec::as_mut_slice returns a mut slice; it does what it says. This name is preferred over as_slice_mut.
    ///
    /// [Ad-hoc conversions follow as_, to_, into_ conventions](https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv)
    C_CONV,
    /// With a few exceptions, the get_ prefix is not used for getters in Rust code.
    ///
    /// The get naming is used only when there is a single and obvious thing that could reasonably be gotten by a getter. For example [Cell::get](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.get) accesses the content of a Cell.
    ///
    /// [Getter names follow Rust convention (C-GETTER)](https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter)
    C_GETTER,
    /// This guideline applies to data structures that are conceptually homogeneous collections. As a counterexample, the str type is slice of bytes that are guaranteed to be valid UTF-8. This is conceptually more nuanced than a homogeneous collection so rather than providing the iter/iter_mut/into_iter group of iterator methods, it provides [str::bytes](https://doc.rust-lang.org/std/primitive.str.html#method.bytes) to iterate as bytes and [str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars) to iterate as chars.
    ///
    /// This guideline applies to methods only, not functions. For example [percent_encode](https://docs.rs/url/1.4.0/url/percent_encoding/fn.percent_encode.html) from the url crate returns an iterator over percent-encoded string fragments. There would be no clarity to be had by using an iter/iter_mut/into_iter convention.
    ///
    /// [Methods on collections that produce iterators follow iter, iter_mut, into_iter (C-ITER)](https://rust-lang.github.io/api-guidelines/naming.html#methods-on-collections-that-produce-iterators-follow-iter-iter_mut-into_iter-c-iter)
    C_ITER,
    /// A method called into_iter() should return a type called IntoIter and similarly for all other methods that return iterators.
    ///
    /// This guideline applies chiefly to methods, but often makes sense for functions as well. For example the [percent_encode](https://docs.rs/url/1.4.0/url/percent_encoding/fn.percent_encode.html) function from the url crate returns an iterator type called PercentEncode.
    ///
    /// These type names make the most sense when prefixed with their owning module, for example [vec::IntoIter](https://doc.rust-lang.org/std/vec/struct.IntoIter.html).
    ///
    /// [Iterator type names match the methods that produce them ](https://rust-lang.github.io/api-guidelines/naming.html#iterator-type-names-match-the-methods-that-produce-them-c-iter-ty)
    C_ITER_TY,
    /// Do not include words in the name of a [Cargo feature](http://doc.crates.io/manifest.html#the-features-section) that convey zero meaning, as in use-abc or with-abc. Name the feature abc directly.
    ///
    /// [Feature names are free of placeholder words (C-FEATURE)](https://rust-lang.github.io/api-guidelines/naming.html#feature-names-are-free-of-placeholder-words-c-feature)
    ///
    C_FEATURE,
    /// All of these use verb-object-error word order. If we were adding an error to represent an address failing to parse, for consistency we would want to name it in verb-object-error order like ParseAddrError rather than AddrParseError.
    ///
    /// [Names use a consistent word order (C-WORD-ORDER)](https://rust-lang.github.io/api-guidelines/naming.html#names-use-a-consistent-word-order-c-word-order)
    C_WORD_ORDER,
}
#[derive(Debug)]
pub enum Interoperability {
    /// Rust's trait system does not allow orphans: roughly, every impl must live either in the crate that defines the trait or the implementing type. Consequently, crates that define new types should eagerly implement all applicable, common traits.
    ///
    /// [Types eagerly implement common traits (C-COMMON-TRAITS)](https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits)
    C_COMMON_TRAITS,
    /// The following conversion traits should be implemented where it makes sense: [From](https://doc.rust-lang.org/std/convert/trait.From.html) [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html) [AsMut](https://doc.rust-lang.org/std/convert/trait.AsMut.html)
    ///
    /// The following conversion traits should never be implemented: [Into](https://doc.rust-lang.org/std/convert/trait.Into.html) [TryInto](https://doc.rust-lang.org/std/convert/trait.TryInto.html).These traits have a blanket impl based on From and TryFrom. Implement those instead.
    ///
    /// Examples from the standard library
    ///
    /// From\<u16\> is implemented for u32 because a smaller integer can always be converted to a bigger integer.
    ///
    /// From\<u32\> is not implemented for u16 because the conversion may not be possible if the integer is too big.
    ///
    /// TryFrom\<u32\> is implemented for u16 and returns an error if the integer is too big to fit in u16.
    ///
    /// [From\<Ipv6Addr\>](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html) is implemented for [IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html), which is a type that can represent both v4 and v6 IP addresses.
    ///
    /// [Conversions use the standard traits From, AsRef, AsMut (C-CONV-TRAITS)](https://rust-lang.github.io/api-guidelines/interoperability.html#conversions-use-the-standard-traits-from-asref-asmut-c-conv-traits)
    ///
    C_CONV_TRAITS,
    ///
    /// [FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) and [Extend](https://doc.rust-lang.org/std/iter/trait.Extend.html) enable collections to be used conveniently with the following iterator methods:
    ///
    /// [Iterator::collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
    ///
    /// [Iterator::partition](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition)
    ///
    /// [Iterator::unzip](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip)
    ///
    /// FromIterator is for creating a new collection containing items from an iterator, and Extend is for adding items from an iterator onto an existing collection.
    ///
    /// Examples from the standard library
    ///
    /// [Vec<T>](https://doc.rust-lang.org/std/vec/struct.Vec.html) implements both FromIterator<T> and Extend<T>.
    ///
    /// [Collections implement FromIterator and Extend (C-COLLECT)](https://rust-lang.github.io/api-guidelines/interoperability.html#collections-implement-fromiterator-and-extend-c-collect)
    C_COLLECT,
    /// Types that play the role of a data structure should implement [Serialize](https://docs.serde.rs/serde/trait.Serialize.html) and [Deserialize](https://docs.serde.rs/serde/trait.Deserialize.html).
    ///
    /// There is a continuum of types between things that are clearly a data structure and things that are clearly not, with gray area in between. [LinkedHashMap](https://docs.rs/linked-hash-map/0.4.2/linked_hash_map/struct.LinkedHashMap.html) and [ IpAddr ](https://doc.rust-lang.org/std/net/enum.IpAddr.html) are data structures. It would be completely reasonable for somebody to want to read in a LinkedHashMap or IpAddr from a JSON file, or send one over IPC to another process. [ LittleEndian ](https://docs.rs/byteorder/1.0.0/byteorder/enum.LittleEndian.html) is not a data structure. It is a marker used by the byteorder crate to optimize at compile time for bytes in a particular order, and in fact an instance of LittleEndian can never exist at runtime. So these are clear-cut examples; the #rust or #serde IRC channels can help assess more ambiguous cases if necessary.
    ///
    /// If a crate does not already depend on Serde for other reasons, it may wish to gate Serde impls behind a Cargo cfg. This way downstream libraries only need to pay the cost of compiling Serde if they need those impls to exist.
    ///
    /// [Data structures implement Serde's Serialize, Deserialize (C-SERDE)](https://rust-lang.github.io/api-guidelines/interoperability.html#data-structures-implement-serdes-serialize-deserialize-c-serde)
    C_SERDE,
    /// [ Send ](https://doc.rust-lang.org/std/marker/trait.Send.html) and [ Sync ](https://doc.rust-lang.org/std/marker/trait.Sync.html) are automatically implemented when the compiler determines it is appropriate.
    ///
    /// In types that manipulate raw pointers, be vigilant that the Send and Sync status of your type accurately reflects its thread safety characteristics. Tests like the following can help catch unintentional regressions in whether the type implements Send or Sync.
    ///
    /// ```
    /// #[test]
    /// fn test_send() {
    ///     fn assert_send<T: Send>() {}
    ///     assert_send::<MyStrangeType>();
    /// }

    /// #[test]
    /// fn test_sync() {
    ///     fn assert_sync<T: Sync>() {}
    ///     assert_sync::<MyStrangeType>();
    /// }
    /// ```
    ///
    /// [Types are Send and Sync where possible (C-SEND-SYNC)](https://rust-lang.github.io/api-guidelines/interoperability.html#types-are-send-and-sync-where-possible-c-send-sync)
    C_SEND_SYNC,
    /// An error type is any type E used in a Result<T, E> returned by any public function of your crate. Error types should always implement the [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html) trait which is the mechanism by which error handling libraries like [error-chain](https://docs.rs/error-chain) abstract over different types of errors, and which allows the error to be used as the [source()](https://doc.rust-lang.org/std/error/trait.Error.html#method.source) of another error.
    ///
    /// Additionally, error types should implement the [ Send ](https://doc.rust-lang.org/std/marker/trait.Send.html) and [ Sync ](https://doc.rust-lang.org/std/marker/trait.Sync.html) traits. An error that is not Send cannot be returned by a thread run with [thread::spawn](https://doc.rust-lang.org/std/thread/fn.spawn.html). An error that is not Sync cannot be passed across threads using an [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html). These are common requirements for basic error handling in a multithreaded application.
    ///
    /// Send and Sync are also important for being able to package a custom error into an IO error using [std::io::Error::new](https://doc.rust-lang.org/std/io/struct.Error.html#method.new), which requires a trait bound of Error + Send + Sync.
    ///
    /// One place to be vigilant about this guideline is in functions that return Error trait objects, for example [reqwest::Error::get_ref](https://docs.rs/reqwest/0.7.2/reqwest/struct.Error.html#method.get_ref). Typically Error + Send + Sync + 'static will be the most useful for callers. The addition of 'static allows the trait object to be used with [Error::downcast_ref](https://doc.rust-lang.org/std/error/trait.Error.html#method.downcast_ref-2).
    ///
    /// Never use () as an error type, even where there is no useful additional information for the error to carry.
    ///
    /// The error message given by the Display representation of an error type should be lowercase without trailing punctuation, and typically concise.
    ///
    /// [Error::description()](https://doc.rust-lang.org/std/error/trait.Error.html#tymethod.description) should not be implemented. It has been deprecated and users should always use Display instead of description() to print the error.
    ///
    /// Examples of error messages
    /// 1. "unexpected end of file"
    /// 2. "provided string was not `true` or `false`"
    /// 3. "invalid IP address syntax"
    /// 4. "second time provided was later than self"
    /// 5. "invalid UTF-8 sequence of {} bytes from index {}"
    /// 6. "environment variable was not valid unicode: {:?}"
    ///
    /// [Error types are meaningful and well-behaved (C-GOOD-ERR)](https://rust-lang.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err)
    C_GOOD_ERR,
    ///
    /// [std::fmt::UpperHex](https://doc.rust-lang.org/std/fmt/trait.UpperHex.html)
    ///
    /// [std::fmt::LowerHex](https://doc.rust-lang.org/std/fmt/trait.LowerHex.html)
    ///
    /// [std::fmt::Octal](https://doc.rust-lang.org/std/fmt/trait.Octal.html)
    ///
    /// [std::fmt::Binary](https://doc.rust-lang.org/std/fmt/trait.Binary.html)
    ///
    /// These traits control the representation of a type under the {:X}, {:x}, {:o}, and {:b} format specifiers.
    ///
    /// [Binary number types provide Hex, Octal, Binary formatting (C-NUM-FMT)](https://rust-lang.github.io/api-guidelines/interoperability.html#binary-number-types-provide-hex-octal-binary-formatting-c-num-fmt)
    C_NUM_FMT,
    /// The standard library contains these two impls:
    /// ```
    /// impl<'a, R: Read + ?Sized> Read for &'a mut R { /* ... */ }

    /// impl<'a, W: Write + ?Sized> Write for &'a mut W { /* ... */ }
    /// ```
    /// That means any function that accepts R: Read or W: Write generic parameters by value can be called with a mut reference if necessary.
    ///
    /// In the documentation of such functions, briefly remind users that a mut reference can be passed. New Rust users often struggle with this. They may have opened a file and want to read multiple pieces of data out of it, but the function to read one piece consumes the reader by value, so they are stuck. The solution would be to leverage one of the above impls and pass &mut f instead of f as the reader parameter.
    ///
    /// Examples
    ///
    /// [flate2::read::GzDecoder::new](https://docs.rs/flate2/0.2/flate2/read/struct.GzDecoder.html#method.new)
    ///
    /// [flate2::write::GzEncoder::new](https://docs.rs/flate2/0.2/flate2/write/struct.GzEncoder.html#method.new)
    ///
    /// [serde_json::from_reader](https://docs.serde.rs/serde_json/fn.from_reader.html)
    ///
    /// [serde_json::to_writer](https://docs.serde.rs/serde_json/fn.to_writer.html)
    ///
    /// [Generic reader/writer functions take R: Read and W: Write by value (C-RW-VALUE)](https://rust-lang.github.io/api-guidelines/interoperability.html#generic-readerwriter-functions-take-r-read-and-w-write-by-value-c-rw-value)
    C_RW_VALUE,
}
pub enum Predictability {
    /// For example, this is why the [Box::into_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw) function is defined the way it is.
    ///
    /// ```
    /// impl<T> Box<T> where T: ?Sized {
    ///     fn into_raw(b: Box<T>) -> *mut T { /* ... */ }
    /// }
    ///let boxed_str: Box<str> = /* ... */;
    ///let ptr = Box::into_raw(boxed_str);
    ///}
    /// ```
    /// If this were defined as an inherent method instead, it would be confusing at the call site whether the method being called is a method on Box<T> or a method on T.
    /// ```
    /// impl<T> Box<T> where T: ?Sized {
    ///     //Do not do this.
    ///     fn into_raw(self) -> *mut T { /* ... */ }
    /// }
    /// let boxed_str: Box<str> = /* ... */;
    /// //This is a method on str accessed through the smart pointer Deref impl.
    /// boxed_str.chars()
    /// //This is a method on Box<str>...?
    /// boxed_str.into_raw()
    /// ```
    /// [Smart pointers do not add inherent methods (C-SMART-PTR)](https://rust-lang.github.io/api-guidelines/predictability.html#smart-pointers-do-not-add-inherent-methods-c-smart-ptr)
    C_SMART_PTR,
    /// When in doubt, prefer to_/as_/into_ to from_, because they are more ergonomic to use (and can be chained with other methods).
    ///
    /// For many conversions between two types, one of the types is clearly more "specific": it provides some additional invariant or interpretation that is not present in the other type. For example, str is more specific than &[u8], since it is a UTF-8 encoded sequence of bytes.
    ///
    /// Conversions should live with the more specific of the involved types. Thus, str provides both the [as_bytes](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes) method and the [from_utf8](https://doc.rust-lang.org/std/str/fn.from_utf8.html) constructor for converting to and from &[u8] values. Besides being intuitive, this convention avoids polluting concrete types like &[u8] with endless conversion methods.
    ///
    /// [Conversions live on the most specific type involved (C-CONV-SPECIFIC)](https://rust-lang.github.io/api-guidelines/predictability.html#conversions-live-on-the-most-specific-type-involved-c-conv-specific)
    C_CONV_SPECIFIC,
    /// for any operation that is clearly associated with a particular type.
    ///
    /// Methods have numerous advantages over functions:
    /// + They do not need to be imported or qualified to be used: all you need is a value of the appropriate type.
    /// + Their invocation performs autoborrowing (including mutable borrows).
    /// + They make it easy to answer the question "what can I do with a value of type T" (especially when using rustdoc).
    /// + They provide self notation, which is more concise and often more clearly conveys ownership distinctions.
    ///
    /// [Functions with a clear receiver are methods (C-METHOD)](https://rust-lang.github.io/api-guidelines/predictability.html#functions-with-a-clear-receiver-are-methods-c-method)
    ///
    C_METHOD,
    /// Prefer
    /// ```
    /// fn foo() -> (Bar, Bar)
    /// ```
    /// Over
    /// ```
    /// fn foo(output: &mut Bar) -> Bar
    /// ```
    /// for returning multiple Bar values.
    ///
    /// Compound return types like tuples and structs are efficiently compiled and do not require heap allocation. If a function needs to return multiple values, it should do so via one of these types.
    ///
    /// The primary exception: sometimes a function is meant to modify data that the caller already owns, for example to re-use a buffer:
    /// ```
    /// fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
    /// ```
    /// [Functions do not take out-parameters (C-NO-OUT)](https://rust-lang.github.io/api-guidelines/predictability.html#functions-do-not-take-out-parameters-c-no-out)
    C_NO_OUT,
    /// Operators with built in syntax (*, |, and so on) can be provided for a type by implementing the traits in [std::ops](https://doc.rust-lang.org/std/ops/index.html#traits). These operators come with strong expectations: implement Mul only for an operation that bears some resemblance to multiplication (and shares the expected properties, e.g. associativity), and so on for the other traits.
    ///
    /// [Operator overloads are unsurprising (C-OVERLOAD)](https://rust-lang.github.io/api-guidelines/predictability.html#operator-overloads-are-unsurprising-c-overload)
    C_OVERLOAD,
    /// The Deref traits are used implicitly by the compiler in many circumstances, and interact with method resolution. The relevant rules are designed specifically to accommodate smart pointers, and so the traits should be used only for that purpose.
    ///
    /// Examples from the standard library
    /// + [Box<T>](https://doc.rust-lang.org/std/boxed/struct.Box.html)
    /// + [String](https://doc.rust-lang.org/std/string/struct.String.html) is a smart pointer to [ str  ](https://doc.rust-lang.org/std/primitive.str.html)
    /// + [Rc<T>](https://doc.rust-lang.org/std/rc/struct.Rc.html)
    /// + [Arc<T>](https://doc.rust-lang.org/std/sync/struct.Arc.html)
    /// + [Cow<'a, T>](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
    ///
    /// [Only smart pointers implement Deref and DerefMut (C-DEREF)](https://rust-lang.github.io/api-guidelines/predictability.html#only-smart-pointers-implement-deref-and-derefmut-c-deref)
    C_DEREF,
    /// In Rust, "constructors" are just a convention. There are a variety of conventions around constructor naming, and the distinctions are often subtle.
    ///
    /// A constructor in its most basic form is a new method with no arguments.
    ///
    /// Constructors are static (no self) inherent methods for the type that they construct. Combined with the practice of fully importing type names, this convention leads to informative but concise construction:
    ///
    /// The name new should generally be used for the primary method of instantiating a type. Sometimes it takes no arguments, as in the examples above. Sometimes it does take arguments, like Box::new which is passed the value to place in the Box.
    ///
    /// Some types' constructors, most notably I/O resource types, use distinct naming conventions for their constructors, as in [File::open](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open), [Mmap::open](https://docs.rs/memmap/0.5.2/memmap/struct.Mmap.html#method.open), [TcpStream::connect](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html#method.connect), and [UdpSocket::bind](https://doc.rust-lang.org/stable/std/net/struct.UdpSocket.html#method.bind). In these cases names are chosen as appropriate for the domain.
    ///
    /// Often there are multiple ways to construct a type. It's common in these cases for secondary constructors to be suffixed _with_foo, as in [Mmap::open_with_offset](https://docs.rs/memmap/0.5.2/memmap/struct.Mmap.html#method.open_with_offset). If your type has a multiplicity of construction options though, consider the builder pattern ([C-BUILDER](https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder)) instead.
    ///
    /// Some constructors are "conversion constructors", methods that create a new type from an existing value of a different type. These typically have names beginning with from_ as in [std::io::Error::from_raw_os_error](https://doc.rust-lang.org/std/io/struct.Error.html#method.from_raw_os_error). Note also though the From trait ([C-CONV-TRAITS](https://rust-lang.github.io/api-guidelines/interoperability.html#c-conv-traits)), which is quite similar. There are three distinctions between a from_-prefixed conversion constructor and a From<T> impl.
    ///
    /// + A from_ constructor can be unsafe; a From impl cannot. One example of this is [Box::from_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw).
    /// + A from_ constructor can accept additional arguments to disambiguate the meaning of the source data, as in [u64::from_str_radix](https://doc.rust-lang.org/std/primitive.u64.html#method.from_str_radix).
    /// + A From impl is only appropriate when the source data type is sufficient to determine the encoding of the output data type. When the input is just a bag of bits like in [u64::from_be](https://doc.rust-lang.org/std/primitive.u64.html#method.from_be) or [String::from_utf8](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8), the conversion constructor name is able to identify their meaning.
    ///
    /// Note that it is common and expected for types to implement both Default and a new constructor. For types that have both, they should have the same behavior. Either one may be implemented in terms of the other.
    ///
    /// Examples from the standard library
    ///
    /// + [std::io::Error::new](https://doc.rust-lang.org/std/io/struct.Error.html#method.new) is the commonly used constructor for an IO error.
    /// + [std::io::Error::from_raw_os_error](https://doc.rust-lang.org/std/io/struct.Error.html#method.from_raw_os_error) is a conversion constructor based on an error code received from the operating system.
    /// + [Box::new](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.new) creates a new container type, taking a single argument.
    /// + [File::open](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.open) opens a file resource.
    /// + [Mmap::open_with_offset](https://docs.rs/memmap/0.5.2/memmap/struct.Mmap.html#method.open_with_offset) opens a memory-mapped file, with additional options.
    ///
    /// [Constructors are static, inherent methods (C-CTOR)](https://rust-lang.github.io/api-guidelines/predictability.html#constructors-are-static-inherent-methods-c-ctor)
    C_CTOR,
}
pub enum Flexibility {
    /// Many functions that answer a question also compute interesting related data. If this data is potentially of interest to the client, consider exposing it in the API.
    ///
    /// Examples from the standard library
    /// + [Vec::binary_search](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.binary_search) does not return a bool of whether the value was found, nor an Option<usize> of the index at which the value was maybe found. Instead it returns information about the index if found, and also the index at which the value would need to be inserted if not found.
    /// + [String::from_utf8](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8) may fail if the input bytes are not UTF-8. In the error case it returns an intermediate result that exposes the byte offset up to which the input was valid UTF-8, as well as handing back ownership of the input bytes.
    /// + [HashMap::insert](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.insert) returns an Option<T> that returns the preexisting value for a given key, if any. For cases where the user wants to recover this value having it returned by the insert operation avoids the user having to do a second hash table lookup.
    ///
    /// [Functions expose intermediate results to avoid duplicate work (C-INTERMEDIATE)](https://rust-lang.github.io/api-guidelines/flexibility.html#functions-expose-intermediate-results-to-avoid-duplicate-work-c-intermediate)
    C_INTERMEDIATE,
    /// If a function requires ownership of an argument, it should take ownership of the argument rather than borrowing and cloning the argument.
    /// ```
    /// // Prefer this:
    /// fn foo(b: Bar) {
    /// /* use b as owned, directly */
    /// }
    /// // Over this:
    /// fn foo(b: &Bar) {
    ///     let b = b.clone();
    ///     /* use b as owned after cloning */
    /// }
    /// ```
    /// If a function does not require ownership of an argument, it should take a shared or exclusive borrow of the argument rather than taking ownership and dropping the argument.
    ///
    /// ```
    /// // Prefer this:
    /// fn foo(b: &Bar) {
    /// /* use b as borrowed */
    /// }
    /// // Over this:
    /// fn foo(b: Bar) {
    /// /* use b as borrowed, it is implicitly dropped before function returns */
    /// }
    /// ```
    /// The <code>Copy</code> trait should only be used as a bound when absolutely needed, not as a way of signaling that copies should be cheap to make.
    ///
    /// [Caller decides where to copy and place data (C-CALLER-CONTROL)](https://rust-lang.github.io/api-guidelines/flexibility.html#caller-decides-where-to-copy-and-place-data-c-caller-control)
    C_CALLER_CONTROL,
    /// The fewer assumptions a function makes about its inputs, the more widely usable it becomes.
    /// ```
    /// //Prefer
    /// fn foo<I: IntoIterator<Item = i64>>(iter: I) { /* ... */ }
    /// //over any of
    /// fn foo(c: &[i64]) { /* ... */ }
    /// fn foo(c: &Vec<i64>) { /* ... */ }
    /// fn foo(c: &SomeOtherCollection<i64>) { /* ... */ }
    /// ```
    /// if the function only needs to iterate over the data.
    ///
    /// More generally, consider using generics to pinpoint the assumptions a function needs to make about its arguments.
    ///
    /// Advantages of generics
    /// + Reusability. Generic functions can be applied to an open-ended collection of types, while giving a clear contract for the functionality those types must provide.

    /// + Static dispatch and optimization. Each use of a generic function is specialized ("monomorphized") to the particular types implementing the trait bounds, which means that (1) invocations of trait methods are static, direct calls to the implementation and (2) the compiler can inline and otherwise optimize these calls.

    /// + Inline layout. If a struct and enum type is generic over some type parameter T, values of type T will be laid out inline in the struct/enum, without any indirection.

    /// + Inference. Since the type parameters to generic functions can usually be inferred, generic functions can help cut down on verbosity in code where explicit conversions or other method calls would usually be necessary.

    /// + Precise types. Because generics give a name to the specific type implementing a trait, it is possible to be precise about places where that exact type is required or produced. For example, a function
    /// ```
    /// fn binary<T: Trait>(x: T, y: T) -> T
    /// ```
    /// is guaranteed to consume and produce elements of exactly the same type T; it cannot be invoked with parameters of different types that both implement Trait.
    ///
    /// Disadvantages of generics
    /// + Code size. Specializing generic functions means that the function body is duplicated. The increase in code size must be weighed against the performance benefits of static dispatch.
    /// + Homogeneous types. This is the other side of the "precise types" coin: if T is a type parameter, it stands for a single actual type. So for example a Vec<T> contains elements of a single concrete type (and, indeed, the vector representation is specialized to lay these out in line). Sometimes heterogeneous collections are useful; see [trait objects](https://rust-lang.github.io/api-guidelines/flexibility.html#c-object).
    /// + Signature verbosity. Heavy use of generics can make it more difficult to read and understand a function's signature.
    ///
    /// Examples from the standard library
    /// + [std::fs::File::open](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) takes an argument of generic type AsRef<Path>. This allows files to be opened conveniently from a string literal "f.txt", a [ Path ](https://doc.rust-lang.org/std/path/struct.Path.html), an [ OsString ](https://doc.rust-lang.org/std/ffi/struct.OsString.html), and a few other types.
    ///
    /// [Functions minimize assumptions about parameters by using generics (C-GENERIC)](https://rust-lang.github.io/api-guidelines/flexibility.html#functions-minimize-assumptions-about-parameters-by-using-generics-c-generic)
    C_GENERIC,
    /// Trait objects have some significant limitations: methods invoked through a trait object cannot use generics, and cannot use Self except in receiver position.
    ///
    /// It is not possible to use generic methods through trait objects, because trait objects require fixed vtable entries, while generic methods produce an unbounded set of functions and cannot be represented as a single vtable entry, even after monomorphization.
    ///
    /// When designing a trait, decide early on whether the trait will be used as an object or as a bound on generics.
    ///
    /// If a trait is meant to be used as an object, its methods should take and return trait objects rather than use generics.
    ///
    /// A where clause of Self: Sized may be used to exclude specific methods from the trait's object. The following trait is not object-safe due to the generic method.
    /// ```
    /// trait MyTrait {
    ///     fn object_safe(&self, i: i32);
    ///     fn not_object_safe<T>(&self, t: T);
    /// }
    /// ```
    /// Adding a requirement of Self: Sized to the generic method excludes it from the trait object and makes the trait object-safe.
    /// ```
    /// trait MyTrait {
    ///     fn object_safe(&self, i: i32);
    ///     fn not_object_safe<T>(&self, t: T) where Self: Sized;
    /// ```
    /// Advantages of trait objects
    /// + Heterogeneity. When you need it, you really need it.
    /// + Code size. Unlike generics, trait objects do not generate specialized (monomorphized) versions of code, which can greatly reduce code size.
    ///
    /// Disadvantages of trait objects
    /// + No generic methods. Trait objects cannot currently provide generic methods.
    /// + Dynamic dispatch and fat pointers. Trait objects inherently involve indirection and vtable dispatch, which can carry a performance penalty.
    /// + No Self. Except for the method receiver argument, methods on trait objects cannot use the Self type.
    ///
    /// Examples from the standard library
    /// + The [io::Read](https://doc.rust-lang.org/std/io/trait.Read.html) and [io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) traits are often used as objects.
    /// + The [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait has several generic methods marked with where Self: Sized to retain the ability to use Iterator as an object.
    ///
    /// [Traits are object-safe if they may be useful as a trait object (C-OBJECT)](https://rust-lang.github.io/api-guidelines/flexibility.html#traits-are-object-safe-if-they-may-be-useful-as-a-trait-object-c-object)
    C_OBJECT,
}
pub enum TypeSafety {
    /// Newtypes can statically distinguish between different interpretations of an underlying type.
    /// For example, a f64 value might be used to represent a quantity in miles or in kilometers. Using newtypes, we can keep track of the intended interpretation:
    /// ```
    /// struct Miles(pub f64);
    /// struct Kilometers(pub f64);
    /// impl Miles {
    ///     fn to_kilometers(self) -> Kilometers { /* ... */ }
    /// }
    /// impl Kilometers {
    ///     fn to_miles(self) -> Miles { /* ... */ }
    /// }
    /// ```
    /// Once we have separated these two types, we can statically ensure that we do not confuse them. For example, the function
    ///
    /// ```
    /// fn are_we_there_yet(distance_travelled: Miles) -> bool { /* ... */ }
    /// ```
    /// cannot accidentally be called with a Kilometers value. The compiler will remind us to perform the conversion, thus averting certain [catastrophic bugs](http://en.wikipedia.org/wiki/Mars_Climate_Orbiter).
    ///
    /// [Newtypes provide static distinctions (C-NEWTYPE)](https://rust-lang.github.io/api-guidelines/type-safety.html#newtypes-provide-static-distinctions-c-newtype)
    C_NEWTYPE,
    ///
    /// ```
    /// // Prefer
    /// let w = Widget::new(Small, Round)
    /// // over
    /// let w = Widget::new(true, false)
    /// ```
    /// Core types like bool, u8 and Option have many possible interpretations.
    ///
    /// Use a deliberate type (whether enum, struct, or tuple) to convey interpretation and invariants. In the above example, it is not immediately clear what true and false are conveying without looking up the argument names, but Small and Round are more suggestive.
    ///
    /// Using custom types makes it easier to expand the options later on, for example by adding an ExtraLarge variant.
    ///
    /// See the newtype pattern ([C-NEWTYPE](https://rust-lang.github.io/api-guidelines/type-safety.html#c-newtype)) for a no-cost way to wrap existing types with a distinguished name.
    ///
    /// [Arguments convey meaning through types, not bool or Option (C-CUSTOM-TYPE)](https://rust-lang.github.io/api-guidelines/type-safety.html#arguments-convey-meaning-through-types-not-bool-or-option-c-custom-type)
    C_CUSTOM_TYPE,
    /// Rust supports enum types with explicitly specified discriminants:
    /// ```
    /// enum Color {
    ///     Red = 0xff0000,
    ///     Green = 0x00ff00,
    ///     Blue = 0x0000ff,
    /// }
    /// ```
    /// Custom discriminants are useful when an enum type needs to be serialized to an integer value compatibly with some other system/language. They support "typesafe" APIs: by taking a Color, rather than an integer, a function is guaranteed to get well-formed inputs, even if it later views those inputs as integers.
    ///
    /// An enum allows an API to request exactly one choice from among many. Sometimes an API's input is instead the presence or absence of a set of flags. In C code, this is often done by having each flag correspond to a particular bit, allowing a single integer to represent, say, 32 or 64 flags. Rust's [bitflags](https://github.com/bitflags/bitflags) crate provides a typesafe representation of this pattern.
    /// ```
    /// use bitflags::bitflags;

    /// bitflags! {
    ///     struct Flags: u32 {
    ///         const FLAG_A = 0b00000001;
    ///         const FLAG_B = 0b00000010;
    ///         const FLAG_C = 0b00000100;
    ///     }
    /// }

    /// fn f(settings: Flags) {
    ///     if settings.contains(Flags::FLAG_A) {
    ///         println!("doing thing A");
    ///     }
    ///     if settings.contains(Flags::FLAG_B) {
    ///         println!("doing thing B");
    ///     }
    ///     if settings.contains(Flags::FLAG_C) {
    ///         println!("doing thing C");
    ///     }
    /// }

    /// fn main() {
    ///     f(Flags::FLAG_A | Flags::FLAG_C);
    /// }
    /// ```
    /// [Types for a set of flags are bitflags, not enums (C-BITFLAG)](https://rust-lang.github.io/api-guidelines/type-safety.html#types-for-a-set-of-flags-are-bitflags-not-enums-c-bitflag)
    C_BITFLAG,
    /// Some data structures are complicated to construct, due to their construction needing:
    /// + a large number of inputs
    /// + compound data (e.g. slices)
    /// + optional configuration data
    /// + choice between several flavors
    ///
    /// which can easily lead to a large number of distinct constructors with many arguments each.
    ///
    /// If T is such a data structure, consider introducing a T builder:
    /// + Introduce a separate data type TBuilder for incrementally configuring a T value. When possible, choose a better name: e.g. [ Command ](https://doc.rust-lang.org/std/process/struct.Command.html) is the builder for a [child process](https://doc.rust-lang.org/std/process/struct.Child.html), [Url](https://docs.rs/url/1.4.0/url/struct.Url.html) can be created from a [ ParseOptions ](https://docs.rs/url/1.4.0/url/struct.ParseOptions.html).
    /// + The builder constructor should take as parameters only the data required to make a T.
    /// + The builder should offer a suite of convenient methods for configuration, including setting up compound inputs (like slices) incrementally. These methods should return self to allow chaining.
    /// + The builder should provide one or more "terminal" methods for actually building a T.
    ///
    /// The builder pattern is especially appropriate when building a T involves side effects, such as spawning a task or launching a process.
    ///
    /// In Rust, there are two variants of the builder pattern, differing in the treatment of ownership, as described below.
    ///
    /// __Non-consuming builders (preferred)__
    ///
    /// In some cases, constructing the final T does not require the builder itself to be consumed. The following variant on std::process::Command is one example
    ///
    /// Note that the spawn method, which actually uses the builder configuration to spawn a process, takes the builder by shared reference. This is possible because spawning the process does not require ownership of the configuration data.
    ///
    /// Because the terminal spawn method only needs a reference, the configuration methods take and return a mutable borrow of self.
    ///
    /// __The benefit__
    ///
    /// By using borrows throughout, Command can be used conveniently for both one-liner and more complex constructions:
    ///
    /// ```
    /// // One-liners
    /// Command::new("/bin/cat").arg("file.txt").spawn();

    /// // Complex configuration
    /// let mut cmd = Command::new("/bin/ls");
    /// if size_sorted {
    ///     cmd.arg("-S");
    /// }
    /// cmd.arg(".");
    /// cmd.spawn();
    /// ```
    /// Consuming builders
    ///
    /// Sometimes builders must transfer ownership when constructing the final type T, meaning that the terminal methods must take self rather than &self.
    ///
    /// When the terminal methods of the builder require ownership, there is a basic tradeoff:
    /// + If the other builder methods take/return a mutable borrow, the complex configuration case will work well, but one-liner configuration becomes impossible.
    /// + If the other builder methods take/return an owned self, one-liners continue to work well but complex configuration is less convenient.
    ///
    /// Under the rubric of making easy things easy and hard things possible, all builder methods for a consuming builder should take and return an owned self. Then client code works as follows:
    /// ```
    /// // One-liners
    /// TaskBuilder::new("my_task").spawn(|| { /* ... */ });
    /// // Complex configuration
    /// let mut task = TaskBuilder::new();
    /// task = task.named("my_task_2"); // must re-assign to retain ownership
    /// if reroute {
    ///     task = task.stdout(mywriter);
    /// }
    /// task.spawn(|| { /* ... */ });
    /// ```
    /// One-liners work as before, because ownership is threaded through each of the builder methods until being consumed by spawn. Complex configuration, however, is more verbose: it requires re-assigning the builder at each step.
    ///
    /// [Builders enable construction of complex values (C-BUILDER)](https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder)
    C_BUILDER,
}
pub enum Dependability {
    /// Rust APIs do not generally follow the [robustness principle](http://en.wikipedia.org/wiki/Robustness_principle): "be conservative in what you send; be liberal in what you accept".
    ///
    /// Instead, Rust code should enforce the validity of input whenever practical.
    ///
    /// Enforcement can be achieved through the following mechanisms (listed in order of preference).
    ///
    /// __Static enforcement__
    ///
    /// Choose an argument type that rules out bad inputs.
    /// ```
    /// // For example, prefer
    /// fn foo(a: Ascii) { /* ... */ }
    /// // over
    /// fn foo(a: u8) { /* ... */ }
    /// ```
    /// where Ascii is a wrapper around u8 that guarantees the highest bit is zero; see newtype patterns ([C-NEWTYPE](https://rust-lang.github.io/api-guidelines/type-safety.html#c-newtype)) for more details on creating typesafe wrappers.
    ///
    /// Static enforcement usually comes at little run-time cost: it pushes the costs to the boundaries (e.g. when a u8 is first converted into an Ascii). It also catches bugs early, during compilation, rather than through run-time failures.
    ///
    /// On the other hand, some properties are difficult or impossible to express using types.
    ///
    /// __Dynamic enforcement__
    ///
    /// Validate the input as it is processed (or ahead of time, if necessary). Dynamic checking is often easier to implement than static checking, but has several downsides:
    /// + Runtime overhead (unless checking can be done as part of processing the input).
    /// + Delayed detection of bugs.
    /// + Introduces failure cases, either via panic! or Result/Option types, which must then be dealt with by client code.
    ///
    /// __Dynamic enforcement with debug_assert!__
    ///
    /// Same as dynamic enforcement, but with the possibility of easily turning off expensive checks for production builds.
    ///
    /// __Dynamic enforcement with opt-out__
    ///
    /// Same as dynamic enforcement, but adds sibling functions that opt out of the checking.
    ///
    /// The convention is to mark these opt-out functions with a suffix like _unchecked or by placing them in a raw submodule.
    ///
    /// The unchecked functions can be used judiciously in cases where (1) performance dictates avoiding checks and (2) the client is otherwise confident that the inputs are valid.
    ///
    /// [Functions validate their arguments (C-VALIDATE)](https://rust-lang.github.io/api-guidelines/dependability.html#functions-validate-their-arguments-c-validate)
    C_VALIDATE,
    /// Destructors are executed while panicking, and in that context a failing destructor causes the program to abort.
    ///
    /// Instead of failing in a destructor, provide a separate method for checking for clean teardown, e.g. a close method, that returns a Result to signal problems. If that close method is not called, the Drop implementation should do the teardown and ignore or log/trace any errors it produces.
    ///
    /// [Destructors never fail (C-DTOR-FAIL)](https://rust-lang.github.io/api-guidelines/dependability.html#destructors-never-fail-c-dtor-fail)
    C_DTOR_FAIL,
    /// Similarly, destructors should not invoke blocking operations, which can make debugging much more difficult. Again, consider providing a separate method for preparing for an infallible, nonblocking teardown.
    ///
    /// [Destructors that may block have alternatives (C-DTOR-BLOCK)](https://rust-lang.github.io/api-guidelines/dependability.html#destructors-that-may-block-have-alternatives-c-dtor-block)
    C_DTOR_BLOCK,
}
pub enum Debuggability {
    /// If there are exceptions, they are rare.
    ///
    /// [All public types implement Debug (C-DEBUG)](https://rust-lang.github.io/api-guidelines/debuggability.html#all-public-types-implement-debug-c-debug)
    C_DEBUG,
    /// Even for conceptually empty values, the Debug representation should never be empty.
    /// ```
    /// let empty_str = "";
    /// assert_eq!(format!("{:?}", empty_str), "\"\"");

    /// let empty_vec = Vec::<bool>::new();
    /// assert_eq!(format!("{:?}", empty_vec), "[]");
    /// ```
    /// [Debug representation is never empty (C-DEBUG-NONEMPTY)](https://rust-lang.github.io/api-guidelines/debuggability.html#debug-representation-is-never-empty-c-debug-nonempty)
    C_DEBUG_NONEMPTY,
}
pub enum FutureProofing {
    /// Some traits are only meant to be implemented within the crate that defines them. In such cases, we can retain the ability to make changes to the trait in a non-breaking way by using the sealed trait pattern.
    /// ```
    /// /// This trait is sealed and cannot be implemented for types outside this crate.
    /// pub trait TheTrait: private::Sealed {
    ///     // Zero or more methods that the user is allowed to call.
    ///     fn ...();
    ///     // Zero or more private methods, not allowed for user to call.
    ///     #[doc(hidden)]
    ///     fn ...();
    /// }
    /// // Implement for some types.
    /// impl TheTrait for usize {
    /// /* ... */
    /// }
    /// mod private {
    ///     pub trait Sealed {}
    ///     // Implement for those same types, but no others.
    ///     impl Sealed for usize {}
    /// }
    /// ```
    /// The empty private Sealed supertrait cannot be named by downstream crates, so we are guaranteed that implementations of Sealed (and therefore TheTrait) only exist in the current crate. We are free to add methods to TheTrait in a non-breaking release even though that would ordinarily be a breaking change for traits that are not sealed. Also we are free to change the signature of methods that are not publicly documented.
    ///
    /// Note that removing a public method or changing the signature of a public method in a sealed trait are still breaking changes.
    ///
    /// To avoid frustrated users trying to implement the trait, it should be documented in rustdoc that the trait is sealed and not meant to be implemented outside of the current crate.
    ///
    /// __Examples__
    /// + [serde_json::value::Index](https://docs.serde.rs/serde_json/value/trait.Index.html)
    /// + [byteorder::ByteOrder](https://docs.rs/byteorder/1.1.0/byteorder/trait.ByteOrder.html)
    ///
    /// [Sealed traits protect against downstream implementations (C-SEALED)](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)
    C_SEALED,
    /// Making a field public is a strong commitment: it pins down a representation choice, and prevents the type from providing any validation or maintaining any invariants on the contents of the field, since clients can mutate it arbitrarily.
    ///
    /// Public fields are most appropriate for struct types in the C spirit: compound, passive data structures. Otherwise, consider providing getter/setter methods and hiding fields instead.
    ///
    /// [Structs have private fields (C-STRUCT-PRIVATE)](https://rust-lang.github.io/api-guidelines/future-proofing.html#structs-have-private-fields-c-struct-private)
    C_STRUCT_PRIVATE,
    /// A newtype can be used to hide representation details while making precise promises to the client.
    ///
    /// For example, consider a function my_transform that returns a compound iterator type.
    /// ```
    /// use std::iter::{Enumerate, Skip};
    /// pub fn my_transform<I: Iterator>(input: I) -> Enumerate<Skip<I>> {
    ///    input.skip(3).enumerate()
    ///}
    /// ```
    /// We wish to hide this type from the client, so that the client's view of the return type is roughly Iterator<Item = (usize, T)>. We can do so using the newtype pattern:
    /// ```
    /// use std::iter::{Enumerate, Skip};
    /// pub struct MyTransformResult<I>(Enumerate<Skip<I>>);
    /// impl<I: Iterator> Iterator for MyTransformResult<I> {
    ///     type Item = (usize, I::Item);
    ///     fn next(&mut self) -> Option<Self::Item> {
    ///         self.0.next()
    ///     }
    /// }
    /// pub fn my_transform<I: Iterator>(input: I) -> MyTransformResult<I> {
    ///     MyTransformResult(input.skip(3).enumerate())
    /// }
    /// ```
    /// Aside from simplifying the signature, this use of newtypes allows us to promise less to the client. The client does not know how the result iterator is constructed or represented, which means the representation can change in the future without breaking client code.
    ///
    /// Rust 1.26 also introduces the [impl Trait](https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md) feature, which is more concise than the newtype pattern but with some additional trade offs, namely with impl Trait you are limited in what you can express. For example, returning an iterator that impls Debug or Clone or some combination of the other iterator extension traits can be problematic. In summary impl Trait as a return type is probably great for internal APIs and may even be appropriate for public APIs, but probably not in all cases. See the "[impl Trait for returning complex types with ease](https://rust-lang.github.io/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html)" section of the Edition Guide for more details.
    /// ```
    /// pub fn my_transform<I: Iterator>(input: I) -> impl Iterator<Item = (usize, I::Item)> {
    ///     input.skip(3).enumerate()
    ///}
    /// ```
    /// [Newtypes encapsulate implementation details (C-NEWTYPE-HIDE)](https://rust-lang.github.io/api-guidelines/future-proofing.html#newtypes-encapsulate-implementation-details-c-newtype-hide)
    C_NEWTYPE_HIDE,
    /// Generic data structures should not use trait bounds that can be derived or do not otherwise add semantic value. Each trait in the derive attribute will be expanded into a separate impl block that only applies to generic arguments that implement that trait.
    /// ```
    /// // Prefer this:
    /// #[derive(Clone, Debug, PartialEq)]
    /// struct Good<T> { /* ... */ }
    /// // Over this:
    /// #[derive(Clone, Debug, PartialEq)]
    /// struct Bad<T: Clone + Debug + PartialEq> { /* ... */ }
    /// ```
    /// Duplicating derived traits as bounds on Bad is unnecessary and a backwards-compatibiliity hazard. To illustrate this point, consider deriving PartialOrd on the structures in the previous example:
    /// ```
    /// // Non-breaking change:
    /// #[derive(Clone, Debug, PartialEq, PartialOrd)]
    /// struct Good<T> { /* ... */ }

    /// // Breaking change:
    /// #[derive(Clone, Debug, PartialEq, PartialOrd)]
    /// struct Bad<T: Clone + Debug + PartialEq + PartialOrd> { /* ... */ }
    /// ```
    /// Generally speaking, adding a trait bound to a data structure is a breaking change because every consumer of that structure will need to start satisfying the additional bound. Deriving more traits from the standard library using the derive attribute is not a breaking change.
    ///
    /// The following traits should never be used in bounds on data structures:
    /// + Clone
    /// + PartialEq
    /// + PartialOrd
    /// + Debug
    /// + Display
    /// + Default
    /// + Error
    /// + Serialize
    /// + Deserialize
    /// + DeserializeOwned
    ///
    /// There is a grey area around other non-derivable trait bounds that are not strictly required by the structure definition, like Read or Write. They may communicate the intended behavior of the type better in its definition but also limits future extensibility. Including semantically useful trait bounds on data structures is still less problematic than including derivable traits as bounds.
    ///
    /// __Exceptions__
    ///
    /// There are three exceptions where trait bounds on structures are required:
    /// + The data structure refers to an associated type on the trait.
    /// + The bound is ?Sized.
    /// + The data structure has a Drop impl that requires trait bounds. Rust currently requires all trait bounds on the Drop impl are also present on the data structure.
    ///
    /// __Examples from the standard library__
    /// + [std::borrow::Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html) refers to an associated type on the Borrow trait.
    /// + [std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html) opts out of the implicit Sized bound.
    /// + [std::io::BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html) requires a trait bound in its Drop impl.
    ///
    /// [Data structures do not duplicate derived trait bounds (C-STRUCT-BOUNDS)]()
    C_STRUCT_BOUNDS,
}
pub enum Necessities {
    /// A crate cannot be stable (>=1.0.0) without all of its public dependencies being stable.
    ///
    /// Public dependencies are crates from which types are used in the public API of the current crate.
    /// ```
    /// pub fn do_my_thing(arg: other_crate::TheirThing) { /* ... */ }
    /// ```
    /// A crate containing this function cannot be stable unless other_crate is also stable.
    ///
    /// Be careful because public dependencies can sneak in at unexpected places.
    ///
    /// [Public dependencies of a stable crate are stable (C-STABLE)](https://rust-lang.github.io/api-guidelines/necessities.html#public-dependencies-of-a-stable-crate-are-stable-c-stable)
    C_STABLE,
    /// The software produced by the Rust project is dual-licensed, under either the [MIT](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) or [Apache 2.0](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE) licenses. Crates that simply need the maximum compatibility with the Rust ecosystem are recommended to do the same, in the manner described herein. Other options are described below.
    ///
    /// These API guidelines do not provide a detailed explanation of Rust's license, but there is a small amount said in the [Rust FAQ](https://github.com/dtolnay/rust-faq#why-a-dual-mitasl2-license). These guidelines are concerned with matters of interoperability with Rust, and are not comprehensive over licensing options.
    ///
    /// To apply the Rust license to your project, define the license field in your Cargo.toml as:
    /// ```
    /// [package]
    /// name = "..."
    /// version = "..."
    /// authors = ["..."]
    /// license = "MIT OR Apache-2.0"
    /// ```
    /// Then add the files LICENSE-APACHE and LICENSE-MIT in the repository root, containing the text of the licenses (which you can obtain, for instance, from choosealicense.com, for Apache-2.0 and MIT).
    ///
    /// Besides the dual MIT/Apache-2.0 license, another common licensing approach used by Rust crate authors is to apply a single permissive license such as MIT or BSD. This license scheme is also entirely compatible with Rust's, because it imposes the minimal restrictions of Rust's MIT license.
    ///
    /// Crates that desire perfect license compatibility with Rust are not recommended to choose only the Apache license. The Apache license, though it is a permissive license, imposes restrictions beyond the MIT and BSD licenses that can discourage or prevent their use in some scenarios, so Apache-only software cannot be used in some situations where most of the Rust runtime stack can.
    ///
    /// The license of a crate's dependencies can affect the restrictions on distribution of the crate itself, so a permissively-licensed crate should generally only depend on permissively-licensed crates.
    ///
    /// [Crate and its dependencies have a permissive license (C-PERMISSIVE)](https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive)
    C_PERMISSIVE,
}
pub enum Documentation {
    /// See [RFC 1687](https://github.com/rust-lang/rfcs/pull/1687).
    ///
    /// [Crate level docs are thorough and include examples (C-CRATE-DOC)](https://rust-lang.github.io/api-guidelines/documentation.html#crate-level-docs-are-thorough-and-include-examples-c-crate-doc)
    C_CRATE_DOC,
    /// Every public module, trait, struct, enum, function, method, macro, and type definition should have an example that exercises the functionality.
    ///
    /// This guideline should be applied within reason.
    ///
    /// A link to an applicable example on another item may be sufficient. For example if exactly one function uses a particular type, it may be appropriate to write a single example on either the function or the type and link to it from the other.
    ///
    /// The purpose of an example is not always to show how to use the item. Readers can be expected to understand how to invoke functions, match on enums, and other fundamental tasks. Rather, an example is often intended to show why someone would want to use the item.
    /// ```
    /// // This would be a poor example of using clone(). It mechanically shows *how* to
    /// // call clone(), but does nothing to show *why* somebody would want this.
    /// fn main() {
    ///     let hello = "hello";
    ///     hello.clone();
    /// }
    /// ```
    /// [All items have a rustdoc example (C-EXAMPLE)](https://rust-lang.github.io/api-guidelines/documentation.html#all-items-have-a-rustdoc-example-c-example)
    C_EXAMPLE,
    /// Like it or not, example code is often copied verbatim by users. Unwrapping an error should be a conscious decision that the user needs to make.
    ///
    /// A common way of structuring fallible example code is the following. The lines beginning with # are compiled by cargo test when building the example but will not appear in user-visible rustdoc.
    ///
    /// ```
    //// ```rust
    /// /// # use std::error::Error;
    /// /// #
    /// /// # fn main() -> Result<(), Box<dyn Error>> {
    /// /// your;
    /// /// example?;
    /// /// code;
    /// /// #
    /// /// #     Ok(())
    /// /// # }
    /// ```
    /// [Examples use ?, not try!, not unwrap (C-QUESTION-MARK)](https://rust-lang.github.io/api-guidelines/documentation.html#examples-use--not-try-not-unwrap-c-question-mark)
    C_QUESTION_MARK,
    /// Error conditions should be documented in an "Errors" section. This applies to trait methods as well -- trait methods for which the implementation is allowed or expected to return an error should be documented with an "Errors" section.
    ///
    /// For example in the standard library, Some implementations of the [std::io::Read::read](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read) trait method may return an error.
    /// ```
    /// /// Pull some bytes from this source into the specified buffer, returning
    /// /// how many bytes were read.
    /// ///
    /// /// ... lots more info ...
    /// ///
    /// /// # Errors
    /// ///
    /// /// If this function encounters any form of I/O or other error, an error
    /// /// variant will be returned. If an error is returned then it must be
    /// /// guaranteed that no bytes were read.
    /// ```
    /// Panic conditions should be documented in a "Panics" section. This applies to trait methods as well -- traits methods for which the implementation is allowed or expected to panic should be documented with a "Panics" section.
    ///
    /// In the standard library the [Vec::insert](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert) method may panic.
    /// ```
    /// /// Inserts an element at position `index` within the vector, shifting all
    /// /// elements after it to the right.
    /// ///
    /// /// # Panics
    /// ///
    /// /// Panics if `index` is out of bounds.
    /// ```
    /// It is not necessary to document all conceivable panic cases, especially if the panic occurs in logic provided by the caller. For example documenting the Display panic in the following code seems excessive. But when in doubt, err on the side of documenting more panic cases.
    ///
    /// ```
    /// /// # Panics
    /// ///
    /// /// This function panics if `T`'s implementation of `Display` panics.
    /// pub fn print<T: Display>(t: T) {
    /// println!("{}", t.to_string());
    /// }
    /// ```
    ///
    /// Unsafe functions should be documented with a "Safety" section that explains all invariants that the caller is responsible for upholding to use the function correctly.
    ///
    /// The unsafe [std::ptr::read](https://doc.rust-lang.org/std/ptr/fn.read.html) requires the following of the caller.
    /// ```
    /// /// Reads the value from `src` without moving it. This leaves the
    /// /// memory in `src` unchanged.
    /// ///
    /// /// # Safety
    /// ///
    /// /// Beyond accepting a raw pointer, this is unsafe because it semantically
    /// /// moves the value out of `src` without preventing further usage of `src`.
    /// /// If `T` is not `Copy`, then care must be taken to ensure that the value at
    /// /// `src` is not used before the data is overwritten again (e.g. with `write`,
    /// /// `zero_memory`, or `copy_memory`). Note that `*src = foo` counts as a use
    /// /// because it will attempt to drop the value previously at `*src`.
    /// ///
    /// /// The pointer must be aligned; use `read_unaligned` if that is not the case.
    /// ```
    /// [Function docs include error, panic, and safety considerations (C-FAILURE)](https://rust-lang.github.io/api-guidelines/documentation.html#function-docs-include-error-panic-and-safety-considerations-c-failure)
    C_FAILURE,
    /// Regular links can be added inline with the usual markdown syntax of [text](url). Links to other types can be added by marking them with [`text`], then adding the link target in a new line at the end of the docstring with [`text`]: <target>, where <target> is described below.
    ///
    /// Link targets to methods within the same type usually look like this:
    /// ```
    /// [`serialize_struct`]: #method.serialize_struct
    /// ```
    /// Link targets to other types usually look like this:
    /// ```
    /// [`Deserialize`]: trait.Deserialize.html
    /// ```
    /// Link targets may also point to a parent or child module:
    /// ```
    /// [`Value`]: ../enum.Value.html
    /// [`DeserializeOwned`]: de/trait.DeserializeOwned.html
    /// ```
    /// This guideline is officially recommended by RFC 1574 under the heading "[Link all the things](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#link-all-the-things)".
    ///
    /// [Prose contains hyperlinks to relevant things (C-LINK)](https://rust-lang.github.io/api-guidelines/documentation.html#prose-contains-hyperlinks-to-relevant-things-c-link)
    C_LINK,
    /// The [package] section of Cargo.toml should include the following values:
    /// + authors
    /// + description
    /// + license
    /// + repository
    /// + keywords
    /// + categories
    ///
    /// In addition, there are two optional metadata fields:
    /// + documentation
    /// + homepage
    ///
    /// By default, crates.io links to documentation for the crate on [docs.rs](https://docs.rs/). The documentation metadata only needs to be set if the documentation is hosted somewhere other than docs.rs, for example because the crate links against a shared library that is not available in the build environment of docs.rs.
    ///
    /// The homepage metadata should only be set if there is a unique website for the crate other than the source repository or API documentation. Do not make homepage redundant with either the documentation or repository values. For example, serde sets homepage to https://serde.rs, a dedicated website.
    ///
    /// [Cargo.toml includes all common metadata (C-METADATA)](https://rust-lang.github.io/api-guidelines/documentation.html#cargotoml-includes-all-common-metadata-c-metadata)
    C_METADATA,
    /// Users of the crate can read the release notes to find a summary of what changed in each published release of the crate. A link to the release notes, or the notes themselves, should be included in the crate-level documentation and/or the repository linked in Cargo.toml.
    ///
    /// Breaking changes (as defined in [RFC 1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)) should be clearly identified in the release notes.
    ///
    /// If using Git to track the source of a crate, every release published to crates.io should have a corresponding tag identifying the commit that was published. A similar process should be used for non-Git VCS tools as well.
    /// ```
    /// # Tag the current commit
    /// GIT_COMMITTER_DATE=$(git log -n1 --pretty=%aD) git tag -a -m "Release 0.3.0" 0.3.0
    /// git push --tags
    /// ```
    /// Annotated tags are preferred because some Git commands ignore unannotated tags if any annotated tags exist.
    ///
    /// __Examples__
    /// + [Serde 1.0.0 release notes](https://github.com/serde-rs/serde/releases/tag/v1.0.0)
    /// + [Serde 0.9.8 release notes](https://github.com/serde-rs/serde/releases/tag/v0.9.8)
    /// + [Serde 0.9.0 release notes](https://github.com/serde-rs/serde/releases/tag/v0.9.0)
    /// + [Diesel change log](https://github.com/diesel-rs/diesel/blob/master/CHANGELOG.md)
    ///
    /// [Release notes document all significant changes (C-RELNOTES)](https://rust-lang.github.io/api-guidelines/documentation.html#release-notes-document-all-significant-changes-c-relnotes)
    C_RELNOTES,
    /// Rustdoc is supposed to include everything users need to use the crate fully and nothing more. It is fine to explain relevant implementation details in prose but they should not be real entries in the documentation.
    ///
    /// Especially be selective about which impls are visible in rustdoc -- all the ones that users would need for using the crate fully, but no others. In the following code the rustdoc of PublicError by default would show the From<PrivateError> impl. We choose to hide it with #[doc(hidden)] because users can never have a PrivateError in their code so this impl would never be relevant to them.
    /// ```
    /// // This error type is returned to users.
    /// pub struct PublicError { /* ... */ }

    /// // This error type is returned by some private helper functions.
    /// struct PrivateError { /* ... */ }

    /// // Enable use of `?` operator.
    /// #[doc(hidden)]
    /// impl From<PrivateError> for PublicError {
    ///     fn from(err: PrivateError) -> PublicError {
    /// /* ... */
    /// }
    /// }
    /// ```
    /// [pub(crate)](https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md) is another great tool for removing implementation details from the public API. It allows items to be used from outside of their own module but not outside of the same crate.
    ///
    /// [Rustdoc does not show unhelpful implementation details (C-HIDDEN)](https://rust-lang.github.io/api-guidelines/documentation.html#rustdoc-does-not-show-unhelpful-implementation-details-c-hidden)
    C_HIDDEN,
}

pub enum Macro {
    /// Rust macros let you dream up practically whatever input syntax you want. Aim to keep input syntax familiar and cohesive with the rest of your users' code by mirroring existing Rust syntax where possible. Pay attention to the choice and placement of keywords and punctuation.
    ///
    /// A good guide is to use syntax, especially keywords and punctuation, that is similar to what will be produced in the output of the macro.
    ///
    /// For example if your macro declares a struct with a particular name given in the input, preface the name with the keyword struct to signal to readers that a struct is being declared with the given name.
    /// ```
    /// // Prefer this...
    /// bitflags! {
    ///     struct S: u32 { /* ... */ }
    /// }

    /// // ...over no keyword...
    /// bitflags! {
    ///     S: u32 { /* ... */ }
    /// }

    /// // ...or some ad-hoc word.
    /// bitflags! {
    ///     flags S: u32 { /* ... */ }
    /// }
    /// ```
    /// Another example is semicolons vs commas. Constants in Rust are followed by semicolons so if your macro declares a chain of constants, they should likely be followed by semicolons even if the syntax is otherwise slightly different from Rust's.
    /// ```
    /// // Ordinary constants use semicolons.
    /// const A: u32 = 0b000001;
    /// const B: u32 = 0b000010;

    /// // So prefer this...
    /// bitflags! {
    ///     struct S: u32 {
    ///         const C = 0b000100;
    ///         const D = 0b001000;
    ///     }
    /// }

    /// // ...over this.
    /// bitflags! {
    ///     struct S: u32 {
    ///         const E = 0b010000,
    ///         const F = 0b100000,
    ///     }
    /// }
    /// ```
    /// Macros are so diverse that these specific examples won't be relevant, but think about how to apply the same principles to your situation.
    ///
    /// [Input syntax is evocative of the output (C-EVOCATIVE)](https://rust-lang.github.io/api-guidelines/macros.html#input-syntax-is-evocative-of-the-output-c-evocative)
    C_EVOCATIVE,
    /// Macros that produce more than one output item should support adding attributes to any one of those items. One common use case would be putting individual items behind a cfg.
    /// ```
    /// bitflags! {
    ///     struct Flags: u8 {
    ///         #[cfg(windows)]
    ///         const ControlCenter = 0b001;
    ///         #[cfg(unix)]
    ///         const Terminal = 0b010;
    ///     }
    /// }
    /// ```
    /// Macros that produce a struct or enum as output should support attributes so that the output can be used with derive.
    /// ```
    /// bitflags! {
    ///     #[derive(Default, Serialize)]
    ///     struct Flags: u8 {
    ///         const ControlCenter = 0b001;
    ///         const Terminal = 0b010;
    ///     }
    /// }
    /// ```

    /// [Item macros compose well with attributes (C-MACRO-ATTR)](https://rust-lang.github.io/api-guidelines/macros.html#item-macros-compose-well-with-attributes-c-macro-attr)
    C_MACRO_ATTR,
    /// Rust allows items to be placed at the module level or within a tighter scope like a function. Item macros should work equally well as ordinary items in all of these places. The test suite should include invocations of the macro in at least the module scope and function scope.
    /// ```
    /// #[cfg(test)]
    /// mod tests {
    ///     test_your_macro_in_a!(module);
    ///     #[test]
    ///     fn anywhere() {
    ///         test_your_macro_in_a!(function);
    ///     }
    /// }
    /// ```
    ///
    /// As a simple example of how things can go wrong, this macro works great in a module scope but fails in a function scope.
    /// ```
    /// macro_rules! broken {
    ///     ($m:ident :: $t:ident) => {
    ///         pub struct $t;
    ///         pub mod $m {
    ///             pub use super::$t;
    ///         }
    ///     }
    /// }
    /// broken!(m::T); // okay, expands to T and m::T
    /// fn g() {
    ///     broken!(m::U); // fails to compile, super::U refers to the containing module not g
    /// }
    /// ```
    /// [Item macros work anywhere that items are allowed (C-ANYWHERE)](https://rust-lang.github.io/api-guidelines/macros.html#item-macros-work-anywhere-that-items-are-allowed-c-anywhere)
    C_ANYWHERE,
    /// Follow Rust syntax for visibility of items produced by a macro. Private by default, public if pub is specified.
    /// ```
    /// bitflags! {
    ///     struct PrivateFlags: u8 {
    ///         const A = 0b0001;
    ///         const B = 0b0010;
    ///     }
    /// }
    /// bitflags! {
    ///     pub struct PublicFlags: u8 {
    ///         const C = 0b0100;
    ///         const D = 0b1000;
    ///     }
    /// }
    /// ```
    /// [Item macros support visibility specifiers (C-MACRO-VIS)](https://rust-lang.github.io/api-guidelines/macros.html#item-macros-support-visibility-specifiers-c-macro-vis)
    C_MACRO_VIS,
    /// If your macro accepts a type fragment like $t:ty in the input, it should be usable with all of the following:
    /// + Primitives: u8, &str
    /// + Relative paths: m::Data
    /// + Absolute paths: ::base::Data
    /// + Upward relative paths: super::Data
    /// + Generics: Vec<String>
    ///
    /// As a simple example of how things can go wrong, this macro works great with primitives and absolute paths but fails with relative paths.
    /// ```
    /// macro_rules! broken {
    ///     ($m:ident => $t:ty) => {
    ///         pub mod $m {
    ///             pub struct Wrapper($t);
    ///         }
    ///     }
    /// }
    /// broken!(a => u8); // okay
    /// broken!(b => ::std::marker::PhantomData<()>); // okay
    /// struct S;
    /// broken!(c => S); // fails to compile
    /// ```
    ///
    /// [Type fragments are flexible (C-MACRO-TY)](https://rust-lang.github.io/api-guidelines/macros.html#type-fragments-are-flexible-c-macro-ty)
    ///
    C_MACRO_TY,
}
