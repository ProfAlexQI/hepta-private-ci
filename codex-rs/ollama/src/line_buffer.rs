use bytes::BytesMut;
use memchr::memchr;

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct LineBuffer {
    bytes: BytesMut,
    /// Prefix already scanned and known not to contain a newline.
    scanned_len: usize,
}

impl LineBuffer {
    pub(crate) fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub(crate) fn take_line(&mut self) -> Option<BytesMut> {
        let Some(relative_index) = memchr(b'\n', &self.bytes[self.scanned_len..]) else {
            self.scanned_len = self.bytes.len();
            return None;
        };

        let newline_index = self.scanned_len + relative_index;
        let line = self.bytes.split_to(newline_index + 1);
        self.scanned_len = 0;
        Some(line)
    }

    pub(crate) fn take_remaining(&mut self) -> Option<BytesMut> {
        if self.bytes.is_empty() {
            return None;
        }
        self.scanned_len = 0;
        Some(self.bytes.split())
    }

    pub(crate) fn len(&self) -> usize {
        self.bytes.len()
    }
}

#[cfg(test)]
#[path = "line_buffer_tests.rs"]
mod tests;
