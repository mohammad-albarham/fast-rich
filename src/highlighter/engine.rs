//! A tiny backtracking matcher with Python `re` semantics.
//!
//! Rich's highlighters are Python regular expressions that rely on
//! lookbehind (`(?<!\w)`), lazy repetition and backtracking order. The
//! `regex` crate supports neither lookaround nor backtracking, so instead of
//! approximating the patterns we build them from these nodes and match them
//! the way Python's `re` does: leftmost position first, alternatives in
//! order, greedy repeats longest-first and lazy repeats shortest-first.
//!
//! Positions are `char` indices (Python indexes strings by code point).

/// A single-character predicate.
pub(crate) type Class = fn(char) -> bool;

/// A pattern node.
pub(crate) enum Node {
    /// One literal character.
    Char(char),
    /// A literal string.
    Str(&'static str),
    /// One character matching a predicate.
    Class(Class),
    /// Nodes matched one after another.
    Seq(Vec<Node>),
    /// The first alternative (in order) that leads to an overall match.
    Alt(Vec<Node>),
    /// `node{min,max}`, greedy or lazy.
    Repeat {
        node: Box<Node>,
        min: usize,
        max: usize,
        greedy: bool,
    },
    /// Named capture group (index into the pattern's group names).
    Group(usize, Box<Node>),
    /// `\b`
    WordBoundary,
    /// `\B`
    NotWordBoundary,
    /// `(?<!c)` where `c` is one character matching the predicate.
    NotAfter(Class),
    /// `(?:sep[class]+)*` (greedy), matched iteratively.
    ///
    /// Requires `sep` not to match `class` and the pattern to continue with
    /// `sep`: then the only ends that can lead to a match are after whole
    /// runs, and Python tries them most-iterations first, which is the
    /// order used here. Unlike a recursive repeat, its stack use doesn't
    /// grow with the number of segments.
    Segments { sep: char, class: Class },
    /// A zero-width pruning check (see [`Guard`]). The `usize` is a cache
    /// slot unique within the pattern.
    Guard(usize, Guard),
}

/// A cheap *necessary* condition for a pattern to match at a position.
///
/// Guards never change what matches; they skip positions where the full
/// backtracking search is certain to fail. Without them, inputs like
/// 20 000 digits in a row take quadratic time (as they do in Python Rich).
#[derive(Clone, Copy)]
pub(crate) enum Guard {
    /// The maximal run of `class` chars starting here is followed by a
    /// char matching `then`.
    RunThen { class: Class, then: Class },
    /// The char occurs somewhere after this position.
    CharLater(char),
}

/// Recursion budget. Matching is continuation-passing, so every nested
/// node keeps its stack frame until the match finishes. Single-char repeats
/// and [`Node::Segments`] are iterative, so depth is bounded by the
/// pattern's size, not the input's; this limit is only a safety net.
const MAX_DEPTH: usize = 256;

pub(crate) const INF: usize = usize::MAX;

/// Python's `\w` for `str` patterns: Unicode alphanumerics plus `_`.
pub(crate) fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Python's `\d` for `str` patterns: Unicode decimal digits.
pub(crate) fn is_decimal(c: char) -> bool {
    c.is_ascii_digit() || (!c.is_ascii() && c.is_numeric())
}

/// A compiled pattern: a root node plus the names of its groups.
pub(crate) struct Pattern {
    pub root: Node,
    pub names: Vec<&'static str>,
    /// Quick check for whether a match may start at a character.
    pub may_start: fn(char) -> bool,
}

/// One named group span from a match, in char indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GroupSpan {
    pub start: usize,
    pub end: usize,
    pub group: usize,
}

struct Matcher<'t> {
    text: &'t [char],
    caps: Vec<Option<(usize, usize)>>,
    depth: usize,
    /// Per-guard precomputed tables, built on first use.
    guards: Vec<Option<Vec<usize>>>,
}

type Cont<'a, 't> = &'a mut dyn FnMut(&mut Matcher<'t>, usize) -> bool;

impl<'t> Matcher<'t> {
    fn word_at(&self, pos: usize) -> bool {
        self.text.get(pos).is_some_and(|&c| is_word(c))
    }

    fn is_boundary(&self, pos: usize) -> bool {
        let before = pos > 0 && self.word_at(pos - 1);
        before != self.word_at(pos)
    }

    /// Match `node` at `pos`, then call `k` with the end position.
    /// Returns true as soon as some continuation succeeds.
    fn m(&mut self, node: &Node, pos: usize, k: Cont<'_, 't>) -> bool {
        if self.depth >= MAX_DEPTH {
            return false;
        }
        self.depth += 1;
        let result = self.m_inner(node, pos, k);
        self.depth -= 1;
        result
    }

    fn m_inner(&mut self, node: &Node, pos: usize, k: Cont<'_, 't>) -> bool {
        match node {
            Node::Char(c) => self.text.get(pos) == Some(c) && k(self, pos + 1),
            Node::Str(s) => {
                let mut p = pos;
                for c in s.chars() {
                    if self.text.get(p) != Some(&c) {
                        return false;
                    }
                    p += 1;
                }
                k(self, p)
            }
            Node::Class(f) => self.text.get(pos).is_some_and(|&c| f(c)) && k(self, pos + 1),
            Node::Seq(nodes) => self.seq(nodes, pos, k),
            Node::Alt(nodes) => {
                for n in nodes {
                    if self.m(n, pos, k) {
                        return true;
                    }
                }
                false
            }
            Node::Repeat {
                node,
                min,
                max,
                greedy,
            } => match node.as_ref() {
                Node::Char(_) | Node::Class(_) => {
                    self.repeat_single(node, *min, *max, *greedy, pos, k)
                }
                _ => self.repeat(node, *min, *max, *greedy, 0, pos, k),
            },
            Node::Group(i, inner) => {
                let i = *i;
                self.m(inner, pos, &mut |s: &mut Matcher<'t>, end| {
                    let old = s.caps[i];
                    s.caps[i] = Some((pos, end));
                    if k(s, end) {
                        return true;
                    }
                    s.caps[i] = old;
                    false
                })
            }
            Node::WordBoundary => self.is_boundary(pos) && k(self, pos),
            Node::NotWordBoundary => !self.is_boundary(pos) && k(self, pos),
            Node::NotAfter(f) => {
                let blocked = pos > 0 && f(self.text[pos - 1]);
                !blocked && k(self, pos)
            }
            Node::Guard(slot, guard) => self.guard_passes(*slot, *guard, pos) && k(self, pos),
            Node::Segments { sep, class } => {
                let mut ends = vec![pos];
                let mut p = pos;
                while self.text.get(p) == Some(sep)
                    && self.text.get(p + 1).is_some_and(|&c| class(c))
                {
                    p += 2;
                    while self.text.get(p).is_some_and(|&c| class(c)) {
                        p += 1;
                    }
                    ends.push(p);
                }
                ends.into_iter().rev().any(|end| k(self, end))
            }
        }
    }

    fn guard_passes(&mut self, slot: usize, guard: Guard, pos: usize) -> bool {
        if self.guards.len() <= slot {
            self.guards.resize(slot + 1, None);
        }
        let text = self.text;
        let table = self.guards[slot].get_or_insert_with(|| match guard {
            // run_end[i] = first index >= i whose char is not in `class`.
            Guard::RunThen { class, .. } => {
                let mut ends = vec![text.len(); text.len() + 1];
                for i in (0..text.len()).rev() {
                    ends[i] = if class(text[i]) { ends[i + 1] } else { i };
                }
                ends
            }
            Guard::CharLater(c) => vec![text.iter().rposition(|&x| x == c).unwrap_or(usize::MAX)],
        });
        match guard {
            Guard::RunThen { then, .. } => text.get(table[pos]).is_some_and(|&c| then(c)),
            Guard::CharLater(_) => table[0] != usize::MAX && table[0] > pos,
        }
    }

    fn seq(&mut self, nodes: &[Node], pos: usize, k: Cont<'_, 't>) -> bool {
        match nodes.split_first() {
            None => k(self, pos),
            Some((first, rest)) => {
                self.m(first, pos, &mut |s: &mut Matcher<'t>, p| s.seq(rest, p, k))
            }
        }
    }

    /// Repetition of a single-character node, done iteratively.
    fn repeat_single(
        &mut self,
        node: &Node,
        min: usize,
        max: usize,
        greedy: bool,
        pos: usize,
        k: Cont<'_, 't>,
    ) -> bool {
        let accepts = |c: char| match node {
            Node::Char(x) => c == *x,
            Node::Class(f) => f(c),
            _ => false,
        };
        let mut n = 0;
        while n < max && self.text.get(pos + n).is_some_and(|&c| accepts(c)) {
            n += 1;
        }
        if n < min {
            return false;
        }
        if greedy {
            (min..=n).rev().any(|count| k(self, pos + count))
        } else {
            (min..=n).any(|count| k(self, pos + count))
        }
    }

    /// Repetition of a compound node, with backtracking.
    #[allow(clippy::too_many_arguments)]
    fn repeat(
        &mut self,
        node: &Node,
        min: usize,
        max: usize,
        greedy: bool,
        count: usize,
        pos: usize,
        k: Cont<'_, 't>,
    ) -> bool {
        let more = |s: &mut Matcher<'t>, k: Cont<'_, 't>| {
            count < max
                && s.m(node, pos, &mut |s: &mut Matcher<'t>, p| {
                    // An empty iteration can't make progress; stop looping.
                    p != pos && s.repeat(node, min, max, greedy, count + 1, p, k)
                })
        };
        // Order matters: greedy tries one more iteration before stopping,
        // lazy tries stopping before one more iteration.
        let order = if greedy { [true, false] } else { [false, true] };
        order.into_iter().any(|try_more| {
            if try_more {
                more(self, &mut *k)
            } else {
                count >= min && k(self, pos)
            }
        })
    }
}

/// Python `re.finditer` over `text`: non-overlapping matches scanned left to
/// right. Calls `on_match` with every named group that captured a non-empty
/// span, in group-definition order.
pub(crate) fn find_iter(pattern: &Pattern, text: &[char], on_match: &mut dyn FnMut(GroupSpan)) {
    let mut m = Matcher {
        text,
        caps: vec![None; pattern.names.len()],
        depth: 0,
        guards: Vec::new(),
    };
    let mut pos = 0;
    while pos <= text.len() {
        if pos < text.len() && !(pattern.may_start)(text[pos]) {
            pos += 1;
            continue;
        }
        m.caps.iter_mut().for_each(|c| *c = None);
        let mut end = None;
        let matched = m.m(&pattern.root, pos, &mut |_, e| {
            end = Some(e);
            true
        });
        match (matched, end) {
            (true, Some(e)) => {
                for (group, cap) in m.caps.iter().enumerate() {
                    if let Some((s, e)) = *cap {
                        if e > s {
                            on_match(GroupSpan {
                                start: s,
                                end: e,
                                group,
                            });
                        }
                    }
                }
                pos = if e > pos { e } else { pos + 1 };
            }
            _ => pos += 1,
        }
    }
}

// ---------------------------------------------------------------------------
// Builders, so patterns read close to the regex they mirror.
// ---------------------------------------------------------------------------

pub(crate) fn ch(c: char) -> Node {
    Node::Char(c)
}

pub(crate) fn lit(s: &'static str) -> Node {
    Node::Str(s)
}

pub(crate) fn class(f: Class) -> Node {
    Node::Class(f)
}

pub(crate) fn seq(nodes: Vec<Node>) -> Node {
    Node::Seq(nodes)
}

pub(crate) fn alt(nodes: Vec<Node>) -> Node {
    Node::Alt(nodes)
}

pub(crate) fn rep(node: Node, min: usize, max: usize) -> Node {
    Node::Repeat {
        node: Box::new(node),
        min,
        max,
        greedy: true,
    }
}

pub(crate) fn lazy(node: Node, min: usize, max: usize) -> Node {
    Node::Repeat {
        node: Box::new(node),
        min,
        max,
        greedy: false,
    }
}

/// `node?`
pub(crate) fn opt(node: Node) -> Node {
    rep(node, 0, 1)
}

pub(crate) fn group(index: usize, node: Node) -> Node {
    Node::Group(index, Box::new(node))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(pattern: &Pattern, text: &str) -> Vec<(String, &'static str)> {
        let chars: Vec<char> = text.chars().collect();
        let mut out = Vec::new();
        find_iter(pattern, &chars, &mut |g| {
            out.push((
                chars[g.start..g.end].iter().collect(),
                pattern.names[g.group],
            ))
        });
        out
    }

    fn any(_: char) -> bool {
        true
    }

    #[test]
    fn greedy_backtracks_to_last_terminator() {
        // (?P<a><.*>) - greedy body finds the LAST '>'
        let p = Pattern {
            root: group(0, seq(vec![ch('<'), rep(class(any), 0, INF), ch('>')])),
            names: vec!["a"],
            may_start: any,
        };
        assert_eq!(run(&p, "x <a> <b> y"), vec![("<a> <b>".into(), "a")]);
    }

    #[test]
    fn lazy_takes_shortest() {
        let p = Pattern {
            root: group(0, seq(vec![ch('<'), lazy(class(any), 0, INF), ch('>')])),
            names: vec!["a"],
            may_start: any,
        };
        assert_eq!(
            run(&p, "<a> <b>"),
            vec![("<a>".into(), "a"), ("<b>".into(), "a")]
        );
    }

    #[test]
    fn word_boundary_backtracking_like_python() {
        // (?P<n>[0-9]+\.?[0-9]*\b) on "3.5s" matches "3." in Python.
        let p = Pattern {
            root: group(
                0,
                seq(vec![
                    rep(class(|c| c.is_ascii_digit()), 1, INF),
                    opt(ch('.')),
                    rep(class(|c| c.is_ascii_digit()), 0, INF),
                    Node::WordBoundary,
                ]),
            ),
            names: vec!["n"],
            may_start: any,
        };
        assert_eq!(run(&p, "3.5s"), vec![("3.".into(), "n")]);
        assert_eq!(run(&p, "3.5 s"), vec![("3.5".into(), "n")]);
    }

    #[test]
    fn lookbehind_blocks_match() {
        let p = Pattern {
            root: seq(vec![
                Node::NotAfter(is_word),
                group(0, rep(class(|c| c.is_ascii_digit()), 1, INF)),
            ]),
            names: vec!["n"],
            may_start: any,
        };
        assert_eq!(run(&p, "a1 2"), vec![("2".into(), "n")]);
    }

    #[test]
    fn compound_repeat_backtracks() {
        // (/[a-z]+)*/  on "/a/b/c" -> "/a/b/"
        let p = Pattern {
            root: group(
                0,
                seq(vec![
                    rep(
                        seq(vec![
                            ch('/'),
                            rep(class(|c| c.is_ascii_lowercase()), 1, INF),
                        ]),
                        0,
                        INF,
                    ),
                    ch('/'),
                ]),
            ),
            names: vec!["p"],
            may_start: any,
        };
        assert_eq!(run(&p, "/a/b/c"), vec![("/a/b/".into(), "p")]);
    }

    #[test]
    fn deep_input_does_not_overflow() {
        // Run on a small stack to prove the depth budget is safe even in
        // debug builds and on threads with reduced stacks.
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(|| {
                let p = Pattern {
                    root: group(
                        0,
                        seq(vec![
                            Node::Segments {
                                sep: '/',
                                class: |c| c == 'a',
                            },
                            ch('/'),
                        ]),
                    ),
                    names: vec!["p"],
                    may_start: any,
                };
                let text = "/a".repeat(50_000);
                let found = run(&p, &text);
                // Whole path except the trailing segment, like Python.
                assert_eq!(found[0].0.len(), text.len() - 1);
            })
            .unwrap()
            .join()
            .expect("matcher overflowed a 256 KiB stack");
    }
}
