use hpi::{HpiReader, HpiItem};
use std::fs::File;
use std::io::{Read, Write, Result};
use std::process;

static HPI_INPUT: &str = "G:\\lib2\\dev\\projects2\\V3Rocket.hpi";
static HPI_OUTPUT: &str = "G:\\lib2\\dev\\projects2\\out.hpi";

fn main() {
    test_read();

    // test_write();
}

fn test_read() {
    let data = read_file_into_bytes(HPI_INPUT).expect("Failed to read file to bytes");

    let context = HpiReader::read(&data).unwrap_or_else(|err| {
        eprintln!("Error! :(");
        eprintln!("{}", err);
        process::exit(1);
    });

    /*context.root.iter(true).for_each(|item| {
        match item {
            hpi::HpiItem::Directory(dir) => println!("Dir: {}/{}", dir.path, dir.name),
            hpi::HpiItem::Entry(entry) => println!("Entry: {}/{}", entry.path, entry.name),
        }
    });*/

    let entry1 = context.root.iter(true).find_map(|item| {
        if let HpiItem::Entry(entry) = item {
            if entry.name == "arawall.fbi" {
                return Some(entry);
            }
        }

        None
    });

    if let Some(entry1) = entry1 {
        let entry_data = context.extract_file(entry1).unwrap();
        let str = std::str::from_utf8(&entry_data).unwrap();

        println!("{}", str);
    }
    else {
        println!("Couldn't find file :(");
    }
}

/*fn test_write(version_data: hpi::internals::VersionData) {
    println!("{:?}", version_data);

    let data = version_data.pack().expect("Failed to pack VersionData");
    write_bytes_to_file(HPI_OUTPUT, &data).expect("Failed to write bytes to files");
}*/

fn read_file_into_bytes(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn write_bytes_to_file(file_path: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(file_path)?;

    file.write_all(data)?;

    Ok(())
}
