//! A macro checks like "behavior" in elixir language.  

mod inner;

use syn::{parse_macro_input, ItemTrait};

/// Check whether the modules have functions with the same signature as the counterparts of behavior trait.
/// ## Example
///
/// ```rust
/// #[behavior::behavior(modules(en, ja))]
/// trait Behavior {
///     fn greeting() -> &'static str;
/// }
///
/// mod en {
///     pub fn greeting() -> &'static str {
///         "hello"
///     }
/// }
///
/// mod ja {
///     pub fn greeting() -> &'static str {
///         "こんにちは"
///     }
/// }
///
/// #[cfg(feature = "en")]
/// pub use en::*;
///
/// #[cfg(all(not(feature = "en"), feature = "ja"))]
/// pub use ja::*;
/// ```
#[proc_macro_attribute]
pub fn behavior(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_trait = parse_macro_input!(item as ItemTrait);
    inner::behavior_inner(args.into(), &input_trait)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
