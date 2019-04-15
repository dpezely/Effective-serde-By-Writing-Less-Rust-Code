Effective serde By Writing Less Rust Code
=========================================

For **Vancouver Rust meetup** 2019-04-17

Vancouver, BC Canada

## Introduction

The example offered is a simple, realistic example of minimalist JSON file
where:

1. Outer structure is an object (not an array).
2. Top-level keys contain information (not name of structure)
3. Inner values within array indicate mixed categories

Write less Rust code overall simply by making use of various
[serde](https://serde.rs/) attributes, the question-mark operator (`?`) and
`ErrorKind` with various impls of `From` and `Into` traits.

## Presentation

The slide deck is intended to be a stand-alone presentation, so there's
something for those who may have missed the original event.

Slides are in Markdown, suitable for use with [Remark.js](http://remarkjs.com/)
[v0.14.0](https://github.com/gnab/remark/releases/tag/v0.14.0).

The Markdown version of the presentation is [presentation.md](./presentation.md),
and the rendered HTML version is [on RawGit CDN](https://cdn.rawgit.com/dpezely/Effective-serde-By-Writing-Less-Rust-Code/master/presentation.html).
