# Isolanguage-1

This crate implements the [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) standard in Rust. It
also has optional Serde support, by using the `serde` feature:

```
isolanguage-1 = { version = "0.2.0", features = ["serde"] }
```

It only has one type, the `LanguageCode` enum which contains every single ISO 639-1 language.
