use std::ops::Index;
use std::fmt;
use std::fmt::Write;

#[derive(Clone)]
pub struct StrStack {
    data: String,
    ends: Vec<usize>,
}

impl Index<usize> for StrStack {
    type Output = str;
    #[inline]
    fn index(&self, index: usize) -> &str {
        let start = if index == 0 {
            0
        } else {
            self.ends[index-1]
        };
        let end = self.ends[index];
        &self.data[start..end]
    }
}

#[derive(Clone, Copy)]
pub struct Iter<'a> {
    stack: &'a StrStack,
    start: usize,
    end: usize,
}

impl fmt::Debug for StrStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;
    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.start == self.end {
            None
        } else {
            let s = &self.stack[self.start];
            self.start += 1;
            Some(s)
        }
    }

    fn count(self) -> usize {
        self.end - self.start
    }

    fn nth(&mut self, n: usize) -> Option<&'a str> {
        match self.start.checked_add(n) {
            Some(n) if n < self.end => {
                self.start = n;
                self.next()
            },
            _ => {
                self.start = self.end;
                None
            }
        }
    }

    fn last(mut self) -> Option<&'a str> {
        self.next_back()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(&self.stack[self.end])
        }
    }
}

impl<'a> IntoIterator for &'a StrStack {
    type IntoIter = Iter<'a>;
    type Item = &'a str;
    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl StrStack {
    /// Create a new StrStack.
    #[inline]
    pub fn new() -> StrStack {
        StrStack {
            data: String::new(),
            ends: Vec::new(),
        }
    }

    /// Create a new StrStack with the given capacity.
    #[inline]
    pub fn with_capacity(bytes: usize, strings: usize) -> StrStack {
        StrStack {
            data: String::with_capacity(bytes),
            ends: Vec::with_capacity(strings)
        }
    }

    #[inline]
    pub fn push(&mut self, s: &str) -> usize {
        self.data.push_str(s);
        self.ends.push(self.data.len());
        self.len() - 1
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {
            stack: self,
            start: 0,
            end: self.len(),
        }
    }

    #[inline]
    pub fn pop(&mut self) -> bool {
        let popped = self.ends.pop().is_some();
        if let Some(&offset) = self.ends.last() {
            self.data.truncate(offset);
        }
        popped
    }

    #[inline]
    pub fn clear(&mut self) {
        self.ends.clear();
        self.data.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ends.len()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.ends.truncate(len);
        self.data.truncate(*self.ends.last().unwrap_or(&0));
    }

    #[inline]
    pub fn writer(&mut self) -> Writer {
        Writer(self)
    }

    #[inline]
    pub fn write_fmt(&mut self, args: fmt::Arguments) -> usize {
        let mut writer = self.writer();
        let _ = writer.write_fmt(args);
        writer.finish()
    }
}

pub struct Writer<'a>(&'a mut StrStack);
impl<'a> Writer<'a> {
    #[inline]
    pub fn finish(self) -> usize {
        // We push on drop.
        self.0.len()
    }
}
impl<'a> fmt::Write for Writer<'a> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.data.push_str(s);
        Ok(())
    }
    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.data.push(c);
        Ok(())
    }
}

impl<'a> Drop for Writer<'a> {
    fn drop(&mut self) {
        self.0.ends.push(self.0.data.len());
    }
}

#[test]
fn test_basic() {
    let mut stack = StrStack::new();
    let first = stack.push("one");
    let second = stack.push("two");
    let third = stack.push("three");

    assert_eq!(&stack[first], "one");
    assert_eq!(&stack[second], "two");
    assert_eq!(&stack[third], "three");

    assert_eq!(stack.len(), 3);

    assert!(stack.pop());

    assert_eq!(stack.len(), 2);

    assert_eq!(&stack[first], "one");
    assert_eq!(&stack[second], "two");

    assert!(stack.pop());
    assert!(stack.pop());

    assert_eq!(stack.len(), 0);
    assert!(!stack.pop());
}

#[test]
fn test_writer() {
    let mut stack = StrStack::new();
    let first = {
        let mut w = stack.writer();
        write!(w, "{}", "first ").unwrap();
        write!(w, "{}", "second").unwrap();
        w.finish()
    };

    let second = {
        let mut w = stack.writer();
        write!(w, "{}", "third ").unwrap();
        write!(w, "{}", "fourth").unwrap();
        w.finish()
    };
    assert_eq!(&stack[first], "first second");
    assert_eq!(&stack[second], "third fourth");
}
