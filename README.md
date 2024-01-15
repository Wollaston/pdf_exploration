# PDF Exploration

\*Experimental and a work in progress\*

An experimental library exploring representing portions of the PDF standard using Rust.

### Concept

The project is intended as practice working with file types represented using bytes (both UTF-8 and non-UTF-8).

The general idea:

- Read a PDF file in as bytes
- Find the byte-offset of the defined PDF keywords
  - This step will use libraries such as [aho_corasick](https://docs.rs/aho-corasick/latest/aho_corasick/) and [bstr](https://docs.rs/bstr/latest/bstr/) that do not panic when encountering non-UTF-8
- Use [nom](https://docs.rs/nom/latest/nom/) to parse each PDF subsection and map the bytes to defined structs

### Reference

The PDF standard is available online as ISO 32000-2:2020
