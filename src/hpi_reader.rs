use std::{error::Error, fmt};
pub struct HpiReader;

impl HpiReader {
    pub fn read(data: &[u8]) -> Result<(), Box<dyn Error>> {
        /*if data.len() < 8 {
            return HpiReaderError::err("Not enough data to read Version".into());
        }

        let version_buf = &data[..8].try_into()?;
        let version_data = VersionData::unpack(version_buf)?;

        if version_data.version != consts::HPI_VERSION {
            return HpiReaderError::err(format!("Unexpected version: {}", version_data.version));
        }

        if version_data.marker != consts::HPI_MARKER {
            return HpiReaderError::err(format!("Unexpected marker: {}", version_data.marker));
        }

        let header_buf = &data[8..32].try_into()?;
        let header_data = HeaderData::unpack(header_buf)?;

        println!("{}", header_data);*/

        Ok(())
    }
}

#[derive(Debug)]
struct HpiReaderError(String);

impl fmt::Display for HpiReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for HpiReaderError {}

impl HpiReaderError {
    pub fn err(msg: String) -> Result<(), Box<dyn Error>> {
        Err(Box::new(HpiReaderError(msg)))
    }
}
