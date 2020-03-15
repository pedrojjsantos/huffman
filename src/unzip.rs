#[path = "adt_huffman.rs"]
mod adt_huffman;

use adt_huffman::Tree;
use std::fs;
use std::io::{Read, Write};

pub fn decompress(mut file: fs::File) -> std::io::Result<()> {
    let (trash, preorder) = get_header(&mut file).unwrap();
    let huff_tree = build_tree(&preorder, &mut 0);
    let mut new = fs::File::create("tests/decompressed")?;

    write_file(&file, &mut new, &huff_tree, trash as i8)?;

    Ok(())
}

fn get_header(file: &mut fs::File) -> std::io::Result<(u8, Vec<u8>)> {
    let mut header: [u8;2] = [0,0];
    
    file.read(&mut header)?;

    let trash = header[0] >> 5;
    let tree_size = header[0] << 3;
    let mut tree_size = (tree_size as usize) << 5;

    tree_size |= header[1] as usize;
    // println!("header:   {:>0digits$b} {:>0digits$b}", header[0], header[1], digits=8);
    // println!("treesize: {:17b}", tree_size);
    // println!("trash:    {:17b}", trash);
    let mut preorder: Vec<u8> = vec![0;tree_size];

    file.read_exact(&mut preorder)?;

    Ok((trash, preorder))
}

fn build_tree(preorder: &[u8], i: &mut usize) -> Tree {
    let mut tree: Tree;
    
    match preorder[*i] {
        b'*'  => {
            tree = Tree::new(b'*', 0);
            *i += 1; 
            tree.add_left(build_tree(preorder, i));
            tree.add_right(build_tree(preorder, i));
        }
        b'\\' => {
            *i += 1;
            tree = Tree::new(preorder[*i], 0);
            *i += 1;
        }
        _  => {
            tree = Tree::new(preorder[*i], 0);
            *i += 1;
        }
    }

    tree
}

fn write_file(old: &fs::File, new: &mut fs::File, huff_tree: &Tree, trash: i8) -> std::io::Result<()> {
    let file: std::io::Result<Vec<u8>> = old.bytes().collect();
    
    let mut file = file.unwrap();
    let last_byte = file.pop().unwrap();

    let mut byte: [u8;1] = [0];
    let mut tree = huff_tree;

    for i in file {
        // println!("-{:>0d$b}-", i, d=8);
        for j in (0..8).rev() {
            match is_set(j, i) {
                false  => {
                    // print!("0");
                    let left = tree.left.as_ref();
                    tree = left.unwrap();
                }
                true => {
                    // print!("1");
                    let right = tree.right.as_ref();
                    tree = right.unwrap();
                }
            }

            if tree.is_leaf() {
                byte[0] = tree.item;
                // println!(": {}", byte[0] as char);
                new.write(&byte)?;
                tree = huff_tree;
            }
        }
    }
    // println!("-{:>0d$b}-", last_byte, d=8);
    for j in (trash..8).rev() {
        match is_set(j, last_byte) {
            false  => {
                // print!("0");
                let left = tree.left.as_ref();
                tree = left.unwrap();
            }
            true => {
                // print!("1");
                let right = tree.right.as_ref();
                tree = right.unwrap();
            }
        }

        if tree.is_leaf() {
            byte[0] = tree.item;
            // println!(": {}", byte[0] as char);
            new.write(&byte)?;
            tree = huff_tree;
        }
    }
    Ok(())
}

fn is_set(i: i8, x: u8) -> bool {
    let mask = 1 << i as u8;
    x & mask != 0
}