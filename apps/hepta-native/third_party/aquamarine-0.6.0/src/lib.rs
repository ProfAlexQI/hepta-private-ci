//! Hermetic compatibility surface for the two upstream Matrix SDK
//! `#[cfg_attr(doc, aquamarine::aquamarine)]` call sites.
//!
//! The archived upstream 0.6.0 implementation embeds an 11 MiB generated
//! JavaScript bundle and depends on the unmaintained `proc-macro-error2`.
//! Hepta preserves the attribute's compile-time shape without injecting local
//! generated assets or remote script execution. Mermaid code blocks remain
//! ordinary rustdoc code blocks.

extern crate proc_macro;

use proc_macro::TokenStream;

/// Preserve the annotated item unchanged.
#[proc_macro_attribute]
pub fn aquamarine(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
