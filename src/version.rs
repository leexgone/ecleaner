use std::error::Error;

pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub build: Option<String>,
}

impl Version {
    pub fn new(expr: &str) -> Result<Version, Box<dyn Error>> {
        let mut version = Version {
            major: 0,
            minor: 0,
            patch: 0,
            build: None,
        };

        for (i, val) in expr.split('.').enumerate() {
            match i {
                0 => {
                    version.major = val.parse()?;
                }
                1 => {
                    version.minor = val.parse()?;
                }
                2 => {
                    version.patch = val.parse()?;
                }
                3 => {
                    version.build = Some(String::from(val));
                }
                _ => {
                }
            }
        };

        Ok(version)
    }
}