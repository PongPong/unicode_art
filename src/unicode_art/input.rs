use std::io::{self, BufRead, Read, Seek, StdinLock};

#[derive(PartialEq)]
enum StdInStatus {
    Init,
    SeekCurrent,
    SeekStart,
    HeaderSent,
}

pub struct Input<R: BufRead> {
    source: R,
    header: [u8; 16],
    header_offset: usize,
    status: StdInStatus,
}

impl Input<StdinLock<'static>> {
    pub fn stdin(stdin: io::Stdin) -> Input<StdinLock<'static>> {
        Input {
            source: stdin.lock(),
            header: [0; 16],
            header_offset: 0,
            status: StdInStatus::Init,
        }
    }

    // /// Get a reference to the input's source.
    // fn source(&self) -> &dyn BufRead {
    //     self.source.as_ref()
    // }
}

impl Seek for Input<StdinLock<'static>> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        if pos == io::SeekFrom::Current(0) {
            self.status = StdInStatus::SeekCurrent;
        } else if pos == io::SeekFrom::Start(0) {
            self.status = StdInStatus::SeekStart;
        }
        Ok(0)
    }
}

impl Read for Input<StdinLock<'static>> {
    /**
     * Stdin couldn't be seek, a workaround to resend the 16 bytes header
     */
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.status == StdInStatus::SeekCurrent {
            // read 16 bytes header
            let res = self.source.read(buf);
            self.header.copy_from_slice(buf);
            return res;
        } else if self.status == StdInStatus::SeekStart {
            if self.header_offset < self.header.len() {
                let start = self.header_offset;
                let end = self.header.len().min(start + buf.len());
                // println!(
                //     "read head bytes = {}, offset: {}, start = {}, end = {}",
                //     buf.len(),
                //     self.header_offset,
                //     start,
                //     end
                // );
                let write_len = end - start;
                buf[..write_len].copy_from_slice(&self.header[start..end]);
                self.header_offset += write_len;
                if self.header_offset >= self.header.len() {
                    self.status = StdInStatus::HeaderSent;
                }
                return Ok(write_len);
            }
        }
        self.source.read(buf)
    }
}

impl BufRead for Input<StdinLock<'static>> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}
