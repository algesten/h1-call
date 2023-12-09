//! no_std allocation free http library.

// For tests we use std.
#![cfg_attr(not(test), no_std)]

mod call;
pub use call::{method, state, version, Call, CallState, Output, ParseResult, Status};

#[derive(Debug, Clone, Copy)]
pub enum H1Error {
    OutputOverflow,
    ParseError(httparse::Error),
    InvalidHttpVersion,
}

pub type Result<T> = core::result::Result<T, H1Error>;

impl From<httparse::Error> for H1Error {
    fn from(value: httparse::Error) -> Self {
        H1Error::ParseError(value)
    }
}

#[cfg(test)]
mod test {
    use crate::call::ParseResult;
    use crate::call::Status;

    use super::*;

    #[test]
    pub fn test_req() -> Result<()> {
        let mut buf = [0; 1024];

        // Call::new starts a new request. The buffer can be on the stack, heap or anywe you want.
        // It is borrowed until we call .flush().
        let output = Call::new(&mut buf)
            // First we select if this is HTTP/1.0 or HTTP/1.1
            .http_10()
            // Then comes the verb (method) + PATH. The methods are narrowed by the typo only be
            // valid for HTTP/1.0. This writes to the underlying buffer – hence the Res return in
            // case buffer overflows.
            .get("/some-path")?
            // At any point we can release the buffer. This returns `Output`, which we need to
            // write to the underlying transport.
            .flush();

        // Output derefs to `&[u8]`, but if that feels opaque, we can use `as_bytes()`.
        assert_eq!(&*output, b"GET /some-path HTTP/1.0\r\n");
        assert_eq!(output.as_bytes(), b"GET /some-path HTTP/1.0\r\n");

        // Once we have written the output to the underlying transport, we call `ready()`, to
        // get a state we can resume.
        let state = output.ready();

        // `Call::resume` takes the state and continues where we left off before calling `.flush()`.
        // The buffer to borrow can be the same we used initially or not. Subsequent output is
        // written to this buffer.
        let output = Call::resume(state, &mut buf)
            // Headers write to the buffer, hence the Result return.
            .header("accept", "text/plain")?
            .header("x-my-thing", "martin")?
            // Finish writes the header end into the buffer and transitions the state to expect
            // response input.
            // The `.finish()` call is only available for HTTP verbs that have no body.
            .finish()?
            // Again, release the buffer to write to a transport.
            .flush();

        assert_eq!(
            &*output,
            b"accept: text/plain\r\nx-my-thing: martin\r\n\r\n"
        );

        // Resume call using the buffer.
        let mut call = Call::resume(output.ready(), &mut buf);

        // Attempt to parse a bunch of incomplete status lines. `ParseResult::Incomplete`
        // means the state is not progressed.
        const ATTEMPT: &[&[u8]] = &[b"HT", b"HTTP/1.0", b"HTTP/1.0 20"];
        for a in ATTEMPT {
            call = match call.parse_status(a)? {
                ParseResult::Incomplete(c) => c,
                ParseResult::Complete(_, _, _) => unreachable!(),
            };
        }

        // Parse the complete status line. ParseResult::Complete continues the state.
        let ParseResult::Complete(_call, _n, status) = call.parse_status(b"HTTP/1.0 200 OK\r\n")?
        else {
            panic!("Expected complete parse")
        };

        assert_eq!(status, Status(200, Some("OK")));
        Ok(())
    }
}