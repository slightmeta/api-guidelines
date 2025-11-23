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