#[path = "adt_huffman.rs"]
mod adt_huffman;

use adt_huffman::{Tree, Heap};
use std::fs;
use std::io::{Read, Write, Seek, SeekFrom};

pub fn compress(mut file: fs::File) -> std::io::Result<()> {
    let mut frequency: Vec<u64> = vec![0;256];
    let f = Read::by_ref(&mut file);

    for i in f.bytes() {
        let b = i.unwrap() as usize;

        frequency[b] += 1;
    }
    file.seek(SeekFrom::Start(0))?;

    let huff_tree = build_huffman_tree(&frequency);
    let mut preorder: Vec<u8> = Vec::new();

    get_preorder(&huff_tree, &mut preorder);

    let mut binary: Vec<String> = vec![String::new();256];

    {
        let mut buf = String::new();
        new_binary(&huff_tree, &mut binary, &mut buf);
    }

    let mut new = fs::File::create("tests/compressed.huff")?;

    let trash = write_file(&file, &mut new, &binary, &preorder).unwrap();

    write_header(&mut new, trash, preorder.len() as u16)?;

    Ok(())
}

fn build_huffman_tree(frequency: &Vec<u64>) -> Tree {
    let mut heap = Heap::new();

    for (b, f) in frequency.iter().enumerate() {
        if *f > 0 {
            heap.enqueue(Tree::new(b as u8, *f));
        }
    }

    while heap.len() > 1 {
        let l = heap.dequeue().unwrap();
        let r = heap.dequeue().unwrap();
        let mut new = Tree::new(b'*', l.freq + r.freq);

        new.add_left(l);
        new.add_right(r);
        heap.enqueue(new);
    }

    heap.dequeue().unwrap()
}

fn new_binary(tree: &Tree, binary: &mut Vec<String>, buf: &mut String) {
    if tree.is_leaf() {
        let i = tree.item as usize;
        binary[i].clone_from(&buf);
        return;
    }

    let left = tree.left.as_ref().unwrap();
    buf.push('0');
    new_binary(left, binary, buf);
    buf.pop();

    let right = tree.right.as_ref().unwrap();
    buf.push('1');
    new_binary(right, binary, buf);
    buf.pop();
}

fn get_preorder(tree: &Tree, preorder: &mut Vec<u8>) {
    if tree.is_leaf() {
        match tree.item {
            b'*'  => {
                preorder.push(b'\\');
                preorder.push(b'*');
            }
            b'\\' => {
                preorder.push(b'\\');
                preorder.push(b'\\');
            }
            item  => preorder.push(item),
        }
        return;
    }

    preorder.push(tree.item);

    let left  = tree.left.as_ref().unwrap();
    let right = tree.right.as_ref().unwrap();

    get_preorder(left, preorder);
    get_preorder(right, preorder);
}

fn write_file(old: &fs::File, new: &mut fs::File, binary: &Vec<String>, preorder: &Vec<u8>) -> std::io::Result<u8> {
    let mut byte: [u8;1] = [0];
    let mut index: i8 = 7;
    let mut flag: bool = false;

    new.write(&byte)?;
    new.write(&byte)?;
    new.write(preorder)?;

    for i in old.bytes() {
        let i = i.unwrap() as usize;

        for j in binary[i].bytes() {
            if j == b'1' {
                byte[0] = set_bit(index, byte[0]);
            }
            
            flag = true;
            index -= 1;
            
            if index < 0 {
                new.write(&byte)?;
                byte[0] = 0;
                index = 7;
                flag = false;
            }
        }
    }

    if flag {
        new.write(&byte)?;
    }

    let trash = if index == 7 {0} else {index + 1};

    Ok(trash as u8)
}

fn write_header(file: &mut fs::File, trash: u8, tree_size: u16) -> std::io::Result<()> {
    let mut header: [u8;2] = [0,0];
    
    header[0] = trash << 5;
    header[0] |= (tree_size >> 8) as u8;
    header[1] = tree_size as u8;

    file.seek(SeekFrom::Start(0))?;
    file.write(&header)?;

    Ok(())
}

fn set_bit(i: i8, x: u8) -> u8 {
    let mask: u8 = 1 << i;
    x | mask
}