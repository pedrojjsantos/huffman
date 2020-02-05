#[path = "adt_huffman.rs"]
mod adt_huffman;

use adt_huffman::{Heap, Tree};
use std::fs;
use std::io::{Read, Write, Seek, SeekFrom};

pub fn compress(file: &mut std::fs::File) -> std::io::Result<()> {
    let mut frequency: Vec<u8> = vec![0;256];

    {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        for byte in buffer.bytes() {
            let i = byte as usize;
            frequency[i] += 1;
        }
    }
    
    let huffman_tree = build_huffman_tree(&frequency);
    
    let mut binary = vec![String::new();256];
    let mut preorder_tree = String::new();
    
    create_preorder(&huffman_tree, &mut preorder_tree);

    {
        let mut string = String::new();
        create_new_binary(huffman_tree, &mut string, &mut binary);
    }

    let mut new_size: usize = 0;

    for (byte, f) in frequency.iter().enumerate() {
        let size = *f as usize;
        new_size += size * binary[byte].len();
    }

    let mut trash_size = 8 - (new_size % 8) as u8;
    if trash_size == 8 {
        trash_size = 0 as u8;
    }

    let mut new_file = fs::File::create("tests/compressed.huff")?;

    write_header(&mut new_file, preorder_tree, trash_size)?;
    write_new_file(&mut new_file, file, &binary)?;

    Ok(())
}

fn build_huffman_tree(frequency: &Vec<u8>) -> Tree<u8> {
    let mut heap = Heap::new();

    for (i, f) in frequency.iter().enumerate() {
        if *f > 0 {
            let tree = Tree::new(i as u8, *f as i64);
            heap.enqueue(tree);
        }
    }

    if heap.data.len() == 1 {
        let left = heap.dequeue().unwrap();
        let mut new = Tree::new(b'*', left.freq);

        new.add_left(left);

        return new;
    }

    while heap.data.len() > 1 {
        let left = heap.dequeue().unwrap();
        let right = heap.dequeue().unwrap();

        let mut new: Tree<u8> = Tree::new(b'*', left.freq + right.freq);

        new.add_left(left);
        new.add_right(right);

        heap.enqueue(new);
    }

    heap.dequeue().unwrap()
}

fn create_preorder(tree: &Tree<u8>, preorder: &mut String) {
    if tree.is_leaf() {
        match tree.item {
            b'\\' => preorder.push_str("\\\\"),
            b'*'  => preorder.push_str("\\*"),
            x     => preorder.push(x as char),
        }
        return;
    }

    preorder.push('*');

    if tree.left.is_some() {
        let left = tree.left.as_ref();
        create_preorder(left.unwrap(), preorder);
    }
    if tree.right.is_some() {
        let right = tree.right.as_ref();
        create_preorder(right.unwrap(), preorder);
    }
}

fn create_new_binary(tree: Tree<u8>, string: &mut String, binary: &mut Vec<String>) {
    if tree.is_leaf() {
        let i = tree.item as usize;
        binary[i] = string.clone();

        return;
    }
    let len = string.len();

    if tree.left.is_some() {
        string.push('0');
        create_new_binary(*tree.left.unwrap(), string, binary);
    }
    string.truncate(len);
    
    if tree.right.is_some() {
        string.push('1');
        create_new_binary(*tree.right.unwrap(), string, binary);
    }
    string.truncate(len);
}

fn write_header(new_file: &mut fs::File, preorder: String, trash_size: u8) -> std::io::Result<()> {
    let mut header: [u8;2] = [0, 0];

    header[0] = trash_size << 5;
    header[0] |= (preorder.len() >> 8) as u8;
    header[1] = preorder.len() as u8;

    new_file.write(&header[..])?;
    new_file.write(preorder.as_bytes())?;

    Ok(())
}

fn write_new_file(new_file: &mut fs::File, old_file: &mut fs::File, binary: &Vec<String>) -> std::io::Result<()> {
    let mut byte: [u8;1] = [0];
    let mut index: i8 = 7;

    old_file.seek(SeekFrom::Start(0))?;

    for i in old_file.bytes() {
        let b = i.unwrap();
        let b = b as usize;
        
        for j in binary[b].bytes() {
            
            if index == -1 {
                new_file.write(&byte[..])?;
                byte[0] = 0;
                index = 7;
            }
            if j == b'1' {
                byte[0] = set_bit(&byte[0], &index);
            }
            index -= 1;
        }
    }

    if byte[0] != 0 {
        new_file.write(&byte[..])?;
    }
    
    Ok(())
}

fn set_bit(item: &u8, i: &i8) -> u8 {
    let mask: u8 = 1 << *i;
    item | mask
}