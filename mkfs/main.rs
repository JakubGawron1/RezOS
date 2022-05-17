// ENTFS => Entity file system; an entity is a file, a directory and a symlink at the same time
use bincode;
use blocks::{Addr, Inode, Node, SuperBlock};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::mem::size_of;

use crate::config::SECTOR_SIZE;

mod blocks;
mod config;

#[derive(Debug)]
enum MkfsError {
    BadConfig,
    FileNotFound(String),
}

struct MkfsReport {}

struct Image<'b> {
    sb: SuperBlock,
    boot: Vec<u8>,
    nodes: Vec<Node<'b>>,
}

impl<'b> Image<'b> {
    fn new(sb: SuperBlock, boot: Vec<u8>) -> Self {
        Self {
            sb: sb,
            boot: boot,
            nodes: vec![],
        }
    }

    fn build(&mut self, target: &mut Vec<u8>) {
        target.append(&mut self.boot);
        target.append(&mut bincode::serialize(&self.sb).unwrap());
        for node in self.nodes.iter() {
            target.append(&mut Vec::from(unsafe { node.dnode }));
        }
    }
}

fn mkfs(cfg: config::Config) -> Result<MkfsReport, MkfsError> {
    let mut boot = Vec::new();
    match cfg.bootloader {
        config::Target::File(name) => {
            if let Ok(file) = File::open(&name) {
                let mut buf_reader = BufReader::new(file);
                buf_reader.read_to_end(&mut boot).unwrap(); // lmao
            } else {
                return Err(MkfsError::FileNotFound(name));
            }
        }
        config::Target::Raw(data) => boot = data,
        _ => return Err(MkfsError::BadConfig),
    }

    let mut image = Image::new(blocks::SuperBlock::new(1, cfg.block_size), boot);

    assert_eq!(size_of::<Inode>(), SECTOR_SIZE);

    match cfg.output {
        config::Target::File(name) => {
            let mut compact = vec![];
            image.build(&mut compact);
            File::create(&name).unwrap().write(&compact).unwrap();
        }
        _ => return Err(MkfsError::BadConfig),
    }
    Ok(MkfsReport {})
}

fn main() {
    let cfg = config::Config::argload();
    mkfs(cfg).unwrap();
}
