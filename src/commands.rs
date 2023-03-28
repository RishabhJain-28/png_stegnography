use std::{any, fs, path::Path, str::FromStr};

use anyhow::{anyhow, Ok, Result};

use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::{self, Chunk},
    chunk_type::ChunkType,
    png::Png,
};

fn read_png_from_file(file_path: &Path) -> Result<Png> {
    let contents = fs::read(&file_path)?;

    let png = Png::try_from(&contents[..])?;
    Ok(png)
}

pub fn encode(args: EncodeArgs) -> Result<()> {
    //get the file
    let mut png = read_png_from_file(args.file_path.as_path())?;

    //encode the file

    let chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes().to_vec(),
    );
    png.append_chunk(chunk);
    //save the file

    let ouput_path = &args.output_file.unwrap_or(args.file_path);

    fs::write(ouput_path, png.as_bytes())?;

    // Ok(())

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<String> {
    //get the file
    let mut png = read_png_from_file(args.file_path.as_path())?;

    //decode the file

    let chunk = png.chunk_by_type(&args.chunk_type);

    if chunk.is_none() {
        return Err(anyhow!("No chunk with type {} found ", args.chunk_type));
    }

    Ok(chunk.unwrap().data_as_string()?)
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    //get the file
    let mut png = read_png_from_file(args.file_path.as_path())?;

    //remove the chunk

    let chunk = png.remove_chunk(&args.chunk_type);

    if chunk.is_err() {
        return Err(anyhow!("No chunk with type {} found ", args.chunk_type));
    }
    //save the file
    fs::write(args.file_path, png.as_bytes())?;

    Ok(())
}

pub fn get_chunk_types(args: PrintArgs) -> Result<Vec<String>> {
    //get the file
    let mut png = read_png_from_file(args.file_path.as_path())?;

    // find all chunk types
    let chunk_types = png
        .chunks()
        .into_iter()
        .map(|c| c.chunk_type().to_string())
        .collect();

    Ok(chunk_types)
}
