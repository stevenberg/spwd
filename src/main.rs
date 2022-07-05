use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use directories::BaseDirs;
use std::env;

fn main() -> Result<()> {
    let path: Utf8PathBuf = env::current_dir()?.try_into()?;
    let home: Utf8PathBuf = match BaseDirs::new() {
        Some(d) => d.home_dir().to_owned().try_into()?,
        None => bail!("Can't find the home directory"),
    };
    println!("{}", spwd::shortened_path(&path, &home)?);
    Ok(())
}
