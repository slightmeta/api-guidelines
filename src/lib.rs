#![allow(non_camel_case_types)]
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
pub enum Predictability {}
pub enum Flexibility {}
pub enum TypeSafety {}
pub enum Dependability {}
pub enum Debuggability {}
pub enum FutureProofing {}
pub enum Necessities {}
