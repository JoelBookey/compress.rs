use bit_vec::BitVec;
use clap::{clap_derive::*, Parser};
use huffman::tree::{HuffmanTree, pretty_print, push_byte, pop_byte, rev_byte};
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
        Cli::Decompress(args) => {
            let output = decompress(&args.name)?;
            if let Some(out) = args.output {
                std::fs::write(&out, &output)?;
            } else {
                print!("{output}");
            }
        }
    }

    Ok(())
}

fn compress(file: &str) -> Result<BitVec, std::io::Error> {
    let input = std::fs::read_to_string(file)?;
    let tree = HuffmanTree::from_str(&input);
    
    let mut s_tree = tree.deconstructed();
    let r = 8-(s_tree.len() % 8);
    let mut message = tree.encode_message(&input);
    message.append(&mut s_tree);
    for _ in 0..r {
        message.push(false);
    }
    eprintln!("{r}");
    push_byte(&mut message, r as u8);
    Ok(message)
}

fn decompress(file: &str) -> Result<String, std::io::Error> {
    let mut input = read_bits_from_file(file)?;
    let r = rev_byte(pop_byte(&mut input).expect("invalid file")) as usize;
    eprintln!("{r}");
    for _ in 0..r {
        let _ = input.pop().expect("invalid file");
    }

    let tree = HuffmanTree::reconstruct(&mut input);
    Ok(tree.decode_bits(input))

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
        let tree = HuffmanTree::from_str(&MESSAGE.to_string());
        let encrypt = &tree.get_lookup_table();
        let ec = encrypt.get(&b'l').unwrap();
        assert_eq!(tree.get_u8(ec.clone()).unwrap() as char, 'l');
    }

    #[test]
    fn test_tree_str() {
        let tree = HuffmanTree::from_str(MESSAGE);
        let encrypt = tree.encode_message(MESSAGE);
        assert_eq!(tree.decode_bits(encrypt), MESSAGE);
    }

    #[test]
    fn test_tree_big_str() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let tree = HuffmanTree::from_str(&input);
        let encrypt = tree.encode_message(&input);
        assert_eq!(tree.decode_bits(encrypt), input);
    }

    #[test]
    fn test_tree_with_file() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let tree = HuffmanTree::from_str(&input);
        let encrypt = tree.encode_message(&input);
        write_bits_to_file("test_output.txt", &encrypt).unwrap();
        let file = read_bits_from_file("test_output.txt").unwrap();
        assert_eq!(encrypt, file);
        let decrypt = tree.decode_bits(file);
        assert_eq!(input, decrypt);
    }
}
