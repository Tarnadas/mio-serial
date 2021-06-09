//! Async interface to mio-serial via tokio.
//! You need to enable the `tokio` feature to use this.
//!

use crate::{
    ClearBuffer, DataBits, FlowControl, Parity, Serial, SerialPort, SerialPortSettings, StopBits,
};

use futures::ready;
use std::io::{self, Read, Write};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::unix::AsyncFd;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// Serial port I/O struct.
pub struct AsyncSerial {
    io: AsyncFd<Serial>,
}

impl AsyncSerial {
    /// Open serial port from a provided path, using the default reactor.
    pub fn from_path<P>(path: P, settings: &SerialPortSettings) -> io::Result<AsyncSerial>
    where
        P: AsRef<Path>,
    {
        let port = Serial::from_path(path.as_ref(), settings)?;
        let io = AsyncFd::new(port)?;

        Ok(AsyncSerial { io })
    }

    /// Create a pair of pseudo serial terminals using the default reactor
    ///
    /// ## Returns
    /// Two connected, unnamed `Serial` objects.
    ///
    /// ## Errors
    /// Attempting any IO or parameter settings on the slave tty after the master
    /// tty is closed will return errors.
    ///
    pub fn pair() -> crate::Result<(Self, Self)> {
        let (master, slave) = Serial::pair()?;

        let master = AsyncSerial {
            io: AsyncFd::new(master)?,
        };
        let slave = AsyncSerial {
            io: AsyncFd::new(slave)?,
        };
        Ok((master, slave))
    }

    /// Sets the exclusivity of the port
    ///
    /// If a port is exclusive, then trying to open the same device path again
    /// will fail.
    ///
    /// See the man pages for the tiocexcl and tiocnxcl ioctl's for more details.
    ///
    /// ## Errors
    ///
    /// * `Io` for any error while setting exclusivity for the port.
    pub fn set_exclusive(&mut self, exclusive: bool) -> crate::Result<()> {
        self.io.get_mut().set_exclusive(exclusive)
    }

    /// Returns the exclusivity of the port
    ///
    /// If a port is exclusive, then trying to open the same device path again
    /// will fail.
    pub fn exclusive(&self) -> bool {
        self.io.get_ref().exclusive()
    }
}

impl SerialPort for AsyncSerial {
    fn settings(&self) -> SerialPortSettings {
        self.io.get_ref().settings()
    }

    fn name(&self) -> Option<String> {
        self.io.get_ref().name()
    }

    fn baud_rate(&self) -> crate::Result<u32> {
        self.io.get_ref().baud_rate()
    }

    fn data_bits(&self) -> crate::Result<DataBits> {
        self.io.get_ref().data_bits()
    }

    fn flow_control(&self) -> crate::Result<FlowControl> {
        self.io.get_ref().flow_control()
    }

    fn parity(&self) -> crate::Result<Parity> {
        self.io.get_ref().parity()
    }

    fn stop_bits(&self) -> crate::Result<StopBits> {
        self.io.get_ref().stop_bits()
    }

    fn timeout(&self) -> Duration {
        Duration::from_secs(0)
    }

    fn set_all(&mut self, settings: &SerialPortSettings) -> crate::Result<()> {
        self.io.get_mut().set_all(settings)
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> crate::Result<()> {
        self.io.get_mut().set_baud_rate(baud_rate)
    }

    fn set_data_bits(&mut self, data_bits: DataBits) -> crate::Result<()> {
        self.io.get_mut().set_data_bits(data_bits)
    }

    fn set_flow_control(&mut self, flow_control: FlowControl) -> crate::Result<()> {
        self.io.get_mut().set_flow_control(flow_control)
    }

    fn set_parity(&mut self, parity: Parity) -> crate::Result<()> {
        self.io.get_mut().set_parity(parity)
    }

    fn set_stop_bits(&mut self, stop_bits: StopBits) -> crate::Result<()> {
        self.io.get_mut().set_stop_bits(stop_bits)
    }

    fn set_timeout(&mut self, _: Duration) -> crate::Result<()> {
        Ok(())
    }

    fn write_request_to_send(&mut self, level: bool) -> crate::Result<()> {
        self.io.get_mut().write_request_to_send(level)
    }

    fn write_data_terminal_ready(&mut self, level: bool) -> crate::Result<()> {
        self.io.get_mut().write_data_terminal_ready(level)
    }

    fn read_clear_to_send(&mut self) -> crate::Result<bool> {
        self.io.get_mut().read_clear_to_send()
    }

    fn read_data_set_ready(&mut self) -> crate::Result<bool> {
        self.io.get_mut().read_data_set_ready()
    }

    fn read_ring_indicator(&mut self) -> crate::Result<bool> {
        self.io.get_mut().read_ring_indicator()
    }

    fn read_carrier_detect(&mut self) -> crate::Result<bool> {
        self.io.get_mut().read_carrier_detect()
    }

    fn bytes_to_read(&self) -> crate::Result<u32> {
        self.io.get_ref().bytes_to_read()
    }

    fn bytes_to_write(&self) -> crate::Result<u32> {
        self.io.get_ref().bytes_to_write()
    }

    fn clear(&self, buffer_to_clear: ClearBuffer) -> crate::Result<()> {
        self.io.get_ref().clear(buffer_to_clear)
    }

    fn try_clone(&self) -> crate::Result<Box<dyn SerialPort>> {
        self.io.get_ref().try_clone()
    }
}

impl Read for AsyncSerial {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.get_mut().read(buf)
    }
}

impl Write for AsyncSerial {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.io.get_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.get_mut().flush()
    }
}

use std::os::unix::io::{AsRawFd, RawFd};
impl AsRawFd for AsyncSerial {
    fn as_raw_fd(&self) -> RawFd {
        self.io.as_raw_fd()
    }
}

impl AsyncRead for AsyncSerial {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self.io.poll_read_ready(cx))?;
            match guard.try_io(|_| {
                let read = self.io.get_ref().read(buf.initialize_unfilled())?;
                buf.advance(read);
                Ok(())
            }) {
                Ok(result) => return Poll::Ready(result),
                Err(_would_block) => continue,
            }
        }
    }
}

impl AsyncWrite for AsyncSerial {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            let mut guard = ready!(self.io.poll_write_ready(cx))?;
            match guard.try_io(|_| self.io.get_ref().write(buf)) {
                Ok(x) => return Poll::Ready(x),
                Err(_would_block) => continue,
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self.io.poll_write_ready(cx))?;
            match guard.try_io(|_| self.io.get_ref().flush()) {
                Ok(x) => return Poll::Ready(x),
                Err(_would_block) => continue,
            }
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
