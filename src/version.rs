use std::{error::Error, fmt::Display};

#[derive(Debug)]
#[derive(Eq)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub build: Option<String>,
}

impl Version {
    pub fn new(major: usize, minor: usize, patch: usize, build: Option<&str>) -> Version {
        Version {
            major,
            minor,
            patch,
            build: if let Some(text) = build {
                Some(String::from(text))
            } else {
                None
            }
        }
    }

    pub fn parse(expr: &str) -> Result<Version, Box<dyn Error>> {
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

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(suffix) = self.build.as_ref() {
            write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, suffix)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ret = self.major.cmp(&other.major);
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        let ret = self.minor.cmp(&other.minor);
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        let ret = self.build.cmp(&other.build);
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        let self_build = if let Some(build) = self.build.as_ref() {
            build
        } else {
            ""
        };

        let other_build = if let Some(build) = other.build.as_ref() {
            build
        } else {
            ""
        };

        self_build.cmp(other_build)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn test_parse_version() {
        let version = Version::parse("1.2.3").expect("error pasing 1.2.3");
        assert_eq!(Version::new(1, 2, 3, None), version);
        assert_eq!("1.2.3", &version.to_string());

        let version = Version::parse("1.2.3.20210115").expect("error pasing 1.2.3.20210115");
        assert_eq!(Version::new(1, 2, 3, Some("20210115")), version);
        assert_eq!("1.2.3.20210115", &version.to_string());
    }
}