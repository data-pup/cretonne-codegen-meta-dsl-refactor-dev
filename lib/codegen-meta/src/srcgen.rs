//! Source code generator.
//!
//! The `srcgen` module contains generic helper routines and classes for
//! generating source code.

use std::collections::{BTreeMap, HashSet};
use std::io;

static SHIFTWIDTH: usize = 4;

struct IndentedScope {
    fmt: Formatter,
    after: Option<String>,
}

impl IndentedScope {
    fn enter(&mut self) {
        self.fmt.indent_push();
    }

    fn exit(&mut self) {
        self.fmt.indent_pop();
        if let Some(ref s) = self.after {
            self.fmt.line(Some(&s));
        }
    }
}

struct Formatter {
    indent: usize,
    lines: Vec<String>
}

impl Formatter {
    /// Source code formatter class.
    ///
    /// - Collect source code to be written to a file.
    /// - Keep track of indentation.
    ///
    /// Indentation example: FIXUP: Convert example to Rust.
    ///
    ///    >>> f = Formatter()
    ///    >>> f.line('Hello line 1')
    ///    >>> f.writelines()
    ///    Hello line 1
    ///    >>> f.indent_push()
    ///    >>> f.comment('Nested comment')
    ///    >>> f.indent_pop()
    ///    >>> f.format('Back {} again', 'home')
    ///    >>> f.writelines()
    ///    Hello line 1
    ///        // Nested comment
    ///    Back home again
    pub fn new() -> Formatter {
        Formatter {
            indent: 0,
            lines: Vec::new(),
        }
    }

    /// Increase current indentation level by one.
    pub fn indent_push(&mut self) {
        // self.indent = format!("{}{:-2$}", self.indent, " ", SHIFTWIDTH);
        self.indent += 1;
    }

    /// Decrease indentation by one level.
    pub fn indent_pop(&mut self) {
        assert!(self.indent > 0, "Already at top level indentation");
        self.indent -= 1;
    }

    /// Get the current whitespace indentation in the form of a String.
    fn get_indent(&self) -> String {
        format!("{}{:-1$}", " ", self.indent * SHIFTWIDTH)
    }

    /// Get a string containing whitespace outdented one level. Used for
    /// lines of code that are inside a single indented block.
    fn get_outdent(&self) -> String {
        let outdent_level = self.indent + 1;
        format!("{}{:-1$}", " ", outdent_level * SHIFTWIDTH)
    }

    /// Add an indented line.
    pub fn line(&mut self, contents: Option<&str>) {
        let new_line = match contents {
            Some(s) => format!("{}{}\n", self.get_indent(), s),
            None => format!("{}\n", self.get_indent()),
        };
        self.lines.push(new_line);
    }

    /// Emit a line outdented one level.
    pub fn outdented_line(&mut self, s: String) {
        let new_line = format!("{}{}", self.get_outdent(), s);
        self.lines.push(new_line);
    }

    /// Write all lines to `out`, or stdout if `out` is `None`.
    pub fn writelines(&self, out: Option<&io::Write>) -> Result<(), io::Error> {
        match out {
            Some(w) => {
                unimplemented!();
                // FIXUP:
                // self.lines.iter().map(|line| w.write(line))
            },
            None => {
                self.lines.iter().for_each(|line| println!("{}", line));
                Ok(())
            },
        }
    }

    /// Write `self.lines` to a file.
    pub fn update_file(&self, filename: &str, directory: Option<&str>) {
        unimplemented!();
    }

    /// FIXUP: Convert example code into Rust.
    ///
    /// Return a scope object for use with a `with` statement:
    ///
    ///    >>> f = Formatter()
    ///    >>> with f.indented('prefix {', '} suffix'):
    ///    ...     f.line('hello')
    ///    >>> f.writelines()
    ///    prefix {
    ///        hello
    ///    } suffix
    ///
    /// The optional `before` and `after` parameters are surrounding lines
    /// which are *not* indented.
    pub fn indented(&self, before: Option<&str>, after: Option<&str>) -> IndentedScope {
        unimplemented!();
    }

    // TODO: Should this class implement a format trait?

    /// Add one or more lines after stripping common indentation.
    pub fn multi_line(&mut self, s: &str) {
        parse_multiline(s).into_iter().for_each(|l| {
            match l {
                Some(s) => self.line(Some(s.as_str())),
                None => self.line(None),
            }
        })
    }

    /// Add a comment line.
    pub fn comment(&mut self, s: &str) {
        let commented_line = format!("// {}", s);
        self.line(Some(&commented_line));
    }

    /// Add a (multi-line) documentation comment.
    pub fn doc_comment(&mut self, contents: &str) {
        for line_contents in parse_multiline(contents) {
            match line_contents {
                Some(s) => {
                    let commented_line = format!("/// {}", s);
                    self.line(Some(commented_line.as_str()));
                },
                None => self.line(Some("///")),
            };
        }
    }

    /// FIXUP: Convert example code into Rust.
    ///
    /// Add a match expression.
    ///
    /// Example:
    ///
    ///    >>> f = Formatter()
    ///    >>> m = Match('x')
    ///    >>> m.arm('Orange', ['a', 'b'], 'some body')
    ///    >>> m.arm('Yellow', ['a', 'b'], 'some body')
    ///    >>> m.arm('Green', ['a', 'b'], 'different body')
    ///    >>> m.arm('Blue', ['x', 'y'], 'some body')
    ///    >>> f.match(m)
    ///    >>> f.writelines()
    ///    match x {
    ///        Orange { a, b } |
    ///        Yellow { a, b } => {
    ///            some body
    ///        }
    ///        Green { a, b } => {
    ///            different body
    ///        }
    ///        Blue { x, y } => {
    ///            some body
    ///        }
    ///    }
    pub fn add_match(&mut self, m: Match) {
        unimplemented!();
    }
}

/// Compute the indentation of s, or None of an empty line.
fn indent(s: &str) -> Option<usize> {
    if s.is_empty() {
        None
    } else {
        let t = s.trim_left();
        Some(s.len() - t.len())
    }
}


/// Given a multi-line string, split it into a sequence of lines after
/// stripping a common indentation, as described in the "trim" function
/// from PEP 257. This is useful for strings defined with doc strings:
///    >>> parse_multiline('\\n    hello\\n    world\\n')
///    ['hello', 'world']
fn parse_multiline(s: &str) -> Vec<Option<String>> {
    // Convert tabs into spaces.
    let expanded_tab = format!("{:-1$}", " ", SHIFTWIDTH);
    let lines: Vec<String> = s.lines().map(|l| l.replace(" ", &expanded_tab)).collect();

    // Determine minimum indentation, ignoring the first line.
    let min_indent = lines.iter()
        .map(|l| l.trim_left().len())
        .filter(|&i| i > 0)
        .min();

    // TODO: Remove indentation (first line is special)
    // TODO: Strip off trailing and leading blank lines.
    unimplemented!();
}

/// Match formatting class.
///
/// Match objects collect all the information needed to emit a Rust `match`
/// expression, automatically deduplicating overlapping identical arms.
///
/// TODO: Convert the example to Rust
///
/// Example:
///
///    >>> m = Match('x')
///    >>> m.arm('Orange', ['a', 'b'], 'some body')
///    >>> m.arm('Yellow', ['a', 'b'], 'some body')
///    >>> m.arm('Green', ['a', 'b'], 'different body')
///    >>> m.arm('Blue', ['x', 'y'], 'some body')
///    >>> assert(len(m.arms) == 3)
///
/// Note that this class is ignorant of Rust types, and considers two fields
/// with the same name to be equivalent.
struct Match {
    expr: String,
    arms: BTreeMap<(Vec<String>, String), HashSet<String>>,
}

impl Match {
    /// Create a new match statemeht on `expr`.
    fn new(expr: String) -> Match {
        Match {
            expr,
            arms: BTreeMap::new(),
        }
    }

    /// Add an arm to the Match statement.
    fn arm(&mut self, name: &str, fields: Vec<String>, body: &str) {
        let key = (fields, body.to_string());
        let match_arm = self.arms.entry(key).or_insert(HashSet::new());
        match_arm.insert(name.to_string());
    }
}
