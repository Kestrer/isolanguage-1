# Isolanguage-1

This crate implements the [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) standard in Rust. It also has optional Serde support, by using the `serde_support` feature:

```
isolanguage-1 = { version = "0.1.0", features = ["serde_support"] }
```

It only has one type, the `LanguageCode` enum which contains every single ISO 639-1 language.
