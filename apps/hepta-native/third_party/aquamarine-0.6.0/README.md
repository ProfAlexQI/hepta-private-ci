# Hepta aquamarine compatibility source

This directory is a provenance-bound compatibility patch for
`aquamarine 0.6.0`.

The only consumers in the native dependency graph are two Matrix SDK
`#[cfg_attr(doc, aquamarine::aquamarine)]` call sites. The local attribute is
deterministic and returns the annotated item unchanged. Mermaid blocks remain
ordinary rustdoc code blocks.

This intentionally removes:

- the unmaintained `proc-macro-error2` dependency;
- the upstream 11 MiB generated JavaScript bundle;
- local or remote script injection during documentation builds.

Runtime Matrix behavior is unchanged. The precise source identity, modified
file hashes, semantic tradeoff, and verification commands are recorded in
`HEPTA_PROVENANCE.json`.
