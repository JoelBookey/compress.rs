use bit_vec::BitVec;
use clap::{clap_derive::*, Parser};
use huffman::tree::generate_tree;
use std::io::Write;

#[derive(Parser)]
enum Cli {
    Compress(FileArgs),
    Decompress(FileArgs),
}

#[derive(Args)]
struct FileArgs {
    name: String,
    #[arg(short)]
    output: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    match cli {
        Cli::Compress(args) => {
            let output = compress(&args.name)?;
            if let Some(out) = args.output {
                write_bits_to_file(&out, &output)?;
            } else {
                print_bits(&output)?;
            }
        }
        Cli::Decompress(_) => unimplemented!(),
    }

    Ok(())
}

fn compress(file: &str) -> Result<BitVec, std::io::Error> {
    let input = std::fs::read_to_string(file)?;
    let tree = generate_tree(&input);
    Ok(tree.encode_message(&input))
}

fn write_bits_to_file(f_name: &str, v: &BitVec) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(f_name)?;
    file.write_all(v.to_bytes().as_slice())?;

    Ok(())
}

fn print_bits(v: &BitVec) -> Result<(), std::io::Error> {
    let _ = std::io::stdout().write(v.to_bytes().as_slice())?;
    Ok(())
}

fn read_bits_from_file(name: &str) -> Result<BitVec, std::io::Error> {
    Ok(BitVec::from_bytes(std::fs::read(name)?.as_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;
    const MESSAGE: &str = "hello my name is gunther welcome to the valley!";
    #[test]
    fn test_tree_char() {
        let tree = generate_tree(&MESSAGE.to_string());
        let encrypt = &tree.get_lookup_table();
        let ec = encrypt.get(&'l').unwrap();
        assert_eq!(tree.get_char(ec.clone()).unwrap(), 'l');
    }

    #[test]
    fn test_tree_str() {
        let tree = generate_tree(MESSAGE);
        let encrypt = tree.encode_message(MESSAGE);
        assert_eq!(tree.decode_bits(encrypt), MESSAGE);
    }

    #[test]
    fn test_tree_big_str() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let tree = generate_tree(&input);
        let encrypt = tree.encode_message(&input);
        assert_eq!(tree.decode_bits(encrypt), input);
    }

    #[test]
    fn test_tree_with_file() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let tree = generate_tree(&input);
        let encrypt = tree.encode_message(&input);
        write_bits_to_file("test_output.txt", &encrypt).unwrap();
        let file = read_bits_from_file("test_output.txt").unwrap();
        assert_eq!(encrypt, file);
        let decrypt = tree.decode_bits(file);
        assert_eq!(input, decrypt);
    }
}
