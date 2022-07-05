use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use std::path;

pub fn shortened_path(path: &Utf8Path, home: &Utf8Path) -> Result<String> {
    if path == home {
        return Ok(String::from("~"));
    }

    let sep = path::MAIN_SEPARATOR.to_string();

    let (prefix, base) = if path.starts_with(&home) {
        (format!("~{sep}"), home.to_owned())
    } else {
        (sep.clone(), Utf8PathBuf::from(sep.clone()))
    };
    let path = path.strip_prefix(base)?;

    let mut parts: Vec<String> = path
        .components()
        .rev()
        .enumerate()
        .map(|(i, p)| {
            let p = p.as_str();
            if i == 0 {
                p.to_string()
            } else {
                let c = p.chars().next().unwrap();
                if c == '.' {
                    p[0..2].to_string()
                } else {
                    c.to_string()
                }
            }
        })
        .collect();

    parts.reverse();

    Ok(format!("{prefix}{}", parts.join(&sep)))
}

#[cfg(test)]
mod tests {
    use crate::shortened_path;
    use camino::Utf8PathBuf;

    #[test]
    fn home_directory() {
        let path = Utf8PathBuf::from("/Users/test");
        let home = Utf8PathBuf::from("/Users/test");

        let result = shortened_path(&path, &home).unwrap();

        assert_eq!(String::from("~"), result);
    }

    #[test]
    fn home_subdirectory() {
        let path = Utf8PathBuf::from("/Users/test/foo/bar");
        let home = Utf8PathBuf::from("/Users/test");

        let result = shortened_path(&path, &home).unwrap();

        assert_eq!(String::from("~/f/bar"), result);
    }

    #[test]
    fn home_dot_directory() {
        let path = Utf8PathBuf::from("/Users/test/.foo/bar");
        let home = Utf8PathBuf::from("/Users/test");

        let result = shortened_path(&path, &home).unwrap();

        assert_eq!(String::from("~/.f/bar"), result);
    }

    #[test]
    fn root_directory() {
        let path = Utf8PathBuf::from("/");
        let home = Utf8PathBuf::from("/Users/test");

        let result = shortened_path(&path, &home).unwrap();

        assert_eq!(String::from("/"), result);
    }

    #[test]
    fn root_subdirectory() {
        let path = Utf8PathBuf::from("/foo/bar");
        let home = Utf8PathBuf::from("/Users/test");

        let result = shortened_path(&path, &home).unwrap();

        assert_eq!(String::from("/f/bar"), result);
    }
}
