use std::fs;

use miniz_oxide::inflate::decompress_to_vec_zlib;
use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::count,
    number::complete::le_u32,
    IResult,
};

#[derive(Debug)]
struct File<'a> {
    file_name: &'a str,
    // This is not a text JSON but a binary format used by Darkest Dungeon,
    // See https://github.com/robojumper/DarkestDungeonSaveEditor/blob/13fa97a28b9f405a85d93ba05fc9bad269b3c7da/rust/ddsaveedit/ddsavelib/src/file/mod.rs
    json: Vec<u8>,
}

fn parse_entry(input: &[u8]) -> IResult<&[u8], File> {
    let (input, _) = tag("A")(input)?;
    let (input, file_name) = map_res(take(256usize), std::str::from_utf8)(input)?;
    let file_name = file_name.trim_end_matches('\0');
    let (input, _) = tag("B")(input)?;
    let (input, json_size) = le_u32(input)?;
    let (input, _) = tag("C")(input)?;
    let (input, json) = map_res(take(json_size as usize), decompress_to_vec_zlib)(input)?;
    let (input, _) = tag("D")(input)?;

    Ok((input, File { file_name, json }))
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<File>> {
    let (input, _) = tag("DDV1")(input)?;
    let (input, file_count) = le_u32(input)?;
    let (input, files) = count(parse_entry, file_count as usize)(input)?;
    let (input, _) = tag("Z")(input)?;

    Ok((input, files))
}

pub fn decode(input_file: &str, output_dir: &str) -> anyhow::Result<()> {
    let input = std::fs::read(input_file)?;
    let output_dir = std::path::Path::new(output_dir);
    if !output_dir.exists() {
        std::fs::create_dir(output_dir)?;
    }

    let (_, parsed) = parse(&input).map_err(|e| anyhow::anyhow!("Unable to parse file: {e}"))?;

    for file in parsed {
        {
            let output_dir = output_dir.join(file.file_name.rsplitn(2, '/').last().unwrap());
            if !output_dir.exists() {
                fs::create_dir_all(output_dir)?;
            }
        }

        let output_file = output_dir.join(file.file_name);
        println!("Writing file: {:?}", output_file);
        std::fs::write(output_file, file.json)?;
    }

    Ok(())
}
