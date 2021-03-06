use std::{error::Error, fmt::Display, fs, io::ErrorKind, path::PathBuf, usize};

use regex::Regex;

use super::version::Version;

#[derive(Debug)]
pub struct Plugin {
    pub path: PathBuf,
    pub name: String,
    pub version: Version,
}

impl Display for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.version)
    }
}

macro_rules! return_err {
    ($msg:expr) => {
        {
            let e = std::io::Error::new(ErrorKind::Other, $msg);
            return Err(Box::new(e));
        }
    }
}

impl Plugin {
    pub fn new(path: PathBuf) -> Result<Plugin, Box<dyn Error>> {
        let filename: String;
        let name = if path.is_file() {
            path.file_stem()
        } else {
            path.file_name()
        };

        match name {
            Some(stem) => {
                filename = String::from(stem.to_str().unwrap());
            },
            None => {
                return_err!(format!("Error parsing plugin: {}", path.display()));
            }
        }

        let regex = Regex::new("_\\d+[.]\\d+[.]\\d+")?;
        let (name, version) = if let Some(m) = regex.find(&filename) { 
            let plugin_name = &filename[0..m.start()];
            let version_expr = &filename[m.start() + 1..];
            match Version::parse(version_expr) {
                Ok(version) => {
                    (
                        String::from(plugin_name),
                        version,
                    )
                },
                Err(e) => {
                    return_err!(format!("Error parsings plugin `{}`: {}", path.display(), e));
                }
            }
        } else {
            (
                filename, 
                Version::new(0, 0, 0, None)
            )
        };

        Ok(Plugin {
            path,
            name,
            version
        })
    }

    pub fn move_to(&self, target: &PathBuf) -> Result<usize, Box<dyn Error>> {
        let count = Plugin::copy_all(&self.path, target)?;

        self.remove()?;
        
        Ok(count)
    }

    fn copy_all(root: &PathBuf, target: &PathBuf) -> Result<usize, Box<dyn Error>> {
        let mut count: usize = 0;
        let mut dest_path = target.clone();
        dest_path.push(root.file_name().unwrap());

        if root.is_file() {
            fs::copy(&root, &dest_path)?;

            count += 1;
        } else if root.is_dir() {
            if !dest_path.exists() {
                fs::create_dir(&dest_path)?;
            }

            for entry in root.read_dir()? {
                let entry = entry?;
                let sub_path = entry.path();

                count += Plugin::copy_all(&sub_path, &dest_path)?;
            }
        }

        Ok(count)
    }

    fn remove(&self) -> Result<(), Box<dyn Error>> {
        if self.path.is_file() {
            fs::remove_file(&self.path)?;
        } else if self.path.is_dir() {
            fs::remove_dir_all(&self.path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use fs::File;

    use crate::version::Version;

    use super::Plugin;

    #[test]
    fn test_parse_plugin_dir() {
        let filename = "javax.xml.rpc_1.1.0.v201209140446";

        fs::create_dir(filename).unwrap();

        let path: PathBuf = PathBuf::from(filename);
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("javax.xml.rpc", &plugin.name);
        assert_eq!(Version::parse("1.1.0.v201209140446").unwrap(), plugin.version);

        fs::remove_dir(filename).unwrap();
    }

    #[test]
    fn test_parse_plugin_file() {
        let filename = "org.apache.commons.codec_1.13.0.v20200108-0001.jar";

        File::create(filename).unwrap();

        let path: PathBuf = PathBuf::from(filename);
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("org.apache.commons.codec", &plugin.name);
        assert_eq!(Version::parse("1.13.0.v20200108-0001").unwrap(), plugin.version);

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_parse_plugin_dual() {
        let filename = "org.w3c.dom.events_3.0.0.draft20060413_v201105210656.jar";

        File::create(filename).unwrap();

        let path = PathBuf::from(filename);
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("org.w3c.dom.events", &plugin.name);
        assert_eq!(Version::parse("3.0.0.draft20060413_v201105210656").unwrap(), plugin.version);

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_parse_plugin_err01() {
        let filename = "org.eclipse.cdt.core.win32.x86_64_6.0.0.202008310002";

        fs::create_dir(filename).unwrap();

        let path = PathBuf::from(filename);
        let plugin = Plugin::new(path).unwrap();
        assert_eq!("org.eclipse.cdt.core.win32.x86_64", &plugin.name);
        assert_eq!(Version::parse("6.0.0.202008310002").unwrap(), plugin.version);

        fs::remove_dir(filename).unwrap();
    }
}