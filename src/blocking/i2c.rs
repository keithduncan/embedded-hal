//! Blocking I2C API
//!
//! Slave addresses used by this API are 7-bit I2C addresses ranging from 0 to 127.
//!
//! Operations on 10-bit slave addresses are not supported by the API yet (but applications might
//! be able to emulate some operations).

/// Blocking read
pub trait Read {
    /// Error type
    type Error;

    /// Reads enough bytes from slave with `address` to fill `buffer`
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Master: ST SAD+R        MAK    MAK ...    NMAK SP
    /// Slave:           SAK B0     B1     ... BN
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+R` = slave address followed by bit 1 to indicate reading
    /// - `SAK` = slave acknowledge
    /// - `Bi` = ith byte of data
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    /// - `SP` = stop condition
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
}

impl<T: Read> Read for &mut T {
    type Error = <T as Read>::Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        (*self).read(address, buffer)
    }
}

/// Blocking write
pub trait Write {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr`
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Master: ST SAD+W     B0     B1     ... BN     SP
    /// Slave:           SAK    SAK    SAK ...    SAK
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+W` = slave address followed by bit 0 to indicate writing
    /// - `SAK` = slave acknowledge
    /// - `Bi` = ith byte of data
    /// - `SP` = stop condition
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error>;
}

impl<T: Write> Write for &mut T {
    type Error = <T as Write>::Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        (*self).write(addr, bytes)
    }
}

/// Blocking write (iterator version)
#[cfg(feature = "unproven")]
pub trait WriteIter {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr`
    ///
    /// # I2C Events (contract)
    ///
    /// Same as `Write`
    fn write<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8>;
}

#[cfg(feature = "unproven")]
impl<T: WriteIter> WriteIter for &mut T {
    type Error = <T as WriteIter>::Error;

    fn write<B>(&mut self, addr: u8, bytes: B) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8> {
        (*self).write(addr, bytes)
    }
}

/// Blocking write + read
pub trait WriteRead {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// # I2C Events (contract)
    ///
    /// ``` text
    /// Master: ST SAD+W     O0     O1     ... OM     SR SAD+R        MAK    MAK ...    NMAK SP
    /// Slave:           SAK    SAK    SAK ...    SAK          SAK I0     I1     ... IN
    /// ```
    ///
    /// Where
    ///
    /// - `ST` = start condition
    /// - `SAD+W` = slave address followed by bit 0 to indicate writing
    /// - `SAK` = slave acknowledge
    /// - `Oi` = ith outgoing byte of data
    /// - `SR` = repeated start condition
    /// - `SAD+R` = slave address followed by bit 1 to indicate reading
    /// - `Ii` = ith incoming byte of data
    /// - `MAK` = master acknowledge
    /// - `NMAK` = master no acknowledge
    /// - `SP` = stop condition
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>;
}

impl<T: WriteRead> WriteRead for &mut T {
    type Error = <T as WriteRead>::Error;

    fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        (*self).write_read(address, bytes, buffer)
    }
}

/// Blocking write (iterator version) + read
#[cfg(feature = "unproven")]
pub trait WriteIterRead {
    /// Error type
    type Error;

    /// Sends bytes to slave with address `addr` and then reads enough bytes to fill `buffer` *in a
    /// single transaction*
    ///
    /// # I2C Events (contract)
    ///
    /// Same as the `WriteRead` trait
    fn write_iter_read<B>(
        &mut self,
        address: u8,
        bytes: B,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>
        where
        B: IntoIterator<Item = u8>;
}

#[cfg(unproven)]
impl<T: WriteIterRead> WriteIterRead for &mut T {
    type Error = <T as WriteIterRead>::Error;

    fn write_iter_read<B>(&mut self, address: u8, bytes: B, buffer: &mut [u8]) -> Result<(), Self::Error>
    where
        B: IntoIterator<Item = u8> {
        (*self).write_iter_read(address, bytes, buffer)
    }
}
