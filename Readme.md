# Nom[x]it
[nom](https://crates.io/crates/nom) based parser for [xit](https://xit.jotaen.net/) files


https://xit.jotaen.net/syntax-guide has more details about the syntax for these files.

`nom` is library for writing parsers using combinators. The xit files have simple enough format is relatively easy to parse.
This lib crate export a [parser](./src/parser) module which exposes the various chunks encountered in the xit files
eg :
- [checkbox](./src/parser/checkbox.rs) 
- [duedate](./src/parser/duedate.rs) 
- [tag](./src/parser/tag.rs) 
- [multiline items](./src/parser/item.rs) 


There is some error reporting at some places using the [VerboseError](https://docs.rs/nom/latest/nom/error/struct.VerboseError.html). However the coverage is weak and might not suitable to any real world application


## TODO
- [ ] unicode support
- [ ] tags inside brackets
- [ ] better error messages
- [ ] returning raw matched text
- [ ] merge contiguous `ItemContent::Other`
- [ ] groups


#### Footnote
Will not be publishing this crate as it was an outcome of trying to play with nom library and thus is just a try project.
