use anyhow::{anyhow, Context};
use mongodb::options::Compressor;

pub fn parse_comma_seperated_compressors(compressors: &str) -> anyhow::Result<Vec<Compressor>> {
  compressors.split(',').map(parse_compressor).collect()
}

fn parse_compressor(compressor: &str) -> anyhow::Result<Compressor> {
  if compressor.contains("snappy") {
    Ok(Compressor::Snappy)
  } else if compressor.contains("zstd") {
    let level = compressor
      .split('(')
      .collect::<Vec<_>>()
      .get(1)
      .map(|l| l.replace(')', ""))
      .map(|l| {
        let l = l
          .parse::<i32>()
          .context("zstd compression level must be i32")?;
        if !(1..=22).contains(&l) {
          Err(anyhow!(
            "ztd compression level must be between 1 and 22. got {l}"
          ))
        } else {
          Ok(l)
        }
      })
      .transpose()?;
    Ok(Compressor::Zstd { level })
  } else if compressor.contains("zlib") {
    let level = compressor
      .split('(')
      .collect::<Vec<_>>()
      .get(1)
      .map(|l| l.replace(')', ""))
      .map(|l| {
        let l = l
          .parse::<i32>()
          .context("zlib compression level must be i32")?;
        if !(0..=22).contains(&l) {
          Err(anyhow!(
            "ztd compression level must be between 0 and 9. got {l}"
          ))
        } else {
          Ok(l)
        }
      })
      .transpose()?;
    Ok(Compressor::Zlib { level })
  } else {
    Err(anyhow!("unrecognized compressor: {compressor}"))
  }
}
