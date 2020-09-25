use std::io::{Read, Write};

/// Wraps a raw connection to provide message based framing on top.
pub struct Connection<W: Write + Read> {
    socket: W,
}

impl<W: Write + Read> Connection<W> {
    /// Creates a new connection, wrapping the socket.
    pub fn new(socket: W) -> Self {
        Connection { socket }
    }

    /// Sends a message to the otherside.
    pub fn send_message(&mut self, message: &[u8]) -> std::io::Result<()> {
        unimplemented!()
    }

    /// Receives a message, blocks until a full message is received.
    pub fn read_message(&mut self) -> std::io::Result<Vec<u8>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_send_message() -> std::io::Result<()> {
        let mut dummy_socket = Cursor::new(vec![]);
        let mut connection = Connection::new(&mut dummy_socket);
        connection.send_message(b"hello world")?;

        // Test prefixed length
        let buf = dummy_socket.into_inner();
        assert_eq!(&buf[..4], [0_u8, 0, 0, 11].as_ref());
        // Test contents
        assert_eq!(&buf[4..], b"hello world");
        Ok(())
    }

    #[test]
    fn test_receive_message() -> std::io::Result<()> {
        let mut dummy_socket = Cursor::new(vec![]);
        dummy_socket.write_all([0_u8, 0, 0, 11].as_ref())?;
        dummy_socket.write_all(b"hello world")?;
        dummy_socket.write_all([0_u8, 0, 0, 14].as_ref())?;
        dummy_socket.write_all(b"something else")?;
        dummy_socket.set_position(0);

        let mut connection = Connection::new(&mut dummy_socket);

        assert_eq!(connection.read_message()?, b"hello world");
        assert_eq!(connection.read_message()?, b"something else");
        Ok(())
    }
}
