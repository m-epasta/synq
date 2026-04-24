pub struct Scanner<'a> {
    input: &'a [u8],
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
            line: 0,
            col: 0,
        }
    }

    #[inline]
    pub(crate) fn current(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    #[inline]
    pub(crate) fn bump(&mut self) {
        if let Some(&_b) = self.input.get(self.pos) {
            self.pos += 1;
            self.col += 1;
        }
    }

    #[inline]
    pub(crate) fn bump_nl(&mut self) {
        if let Some(&_b) = self.input.get(self.pos) {
            self.pos += 1;
            self.line += 1;
            self.col = 0;
        }
    }

    // whitespaces and comments
    pub(crate) fn skip(&mut self) {
        while let Some(b) = self.current() {
            match b {
                b' ' | b'\t' | b'\r' => self.bump(),
                b'/' => {
                    if self.input.get(self.pos + 1) == Some(&b'/') {
                        self.bump(); // '/'
                        self.bump(); // '/'
                        while let Some(c) = self.current() {
                            if c == b'\n' {
                                break;
                            }
                            self.bump();
                        }
                    } else if self.input.get(self.pos + 1) == Some(&b'*') {
                        self.bump(); // '/'
                        self.bump(); // '*'
                        while let Some(c) = self.current() {
                            self.bump();
                            if c == b'*' && self.current() == Some(b'/') {
                                self.bump(); // '/'
                                break;
                            }
                        }
                    } else {
                        break; // Single slash
                    }
                }
                _ => break,
            }
        }
    }

    #[inline]
    pub(crate) fn curr_loc(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    #[inline]
    pub(crate) fn slice(&self, start: usize, end: usize) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(&self.input[start..end]) }
    }

    #[inline]
    pub(crate) fn current_pos(&self) -> usize {
        self.pos
    }
}
