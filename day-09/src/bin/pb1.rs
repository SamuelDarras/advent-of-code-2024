#[derive(Debug, PartialEq)]
enum BlockType {
    File,
    Empty,
}

#[derive(Debug)]
struct Block {
    block_type: BlockType,
    id: Option<usize>,
    len: u8,
}

fn main() {
    let src = include_str!("src1.txt");
    // let src = "2333133121414131402";
    // let src = "12345";

    let mut disk = src
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c as u8 - '0' as u8;
            let (id, block_type) = if i % 2 == 0 {
                (Some(i / 2), BlockType::File)
            } else {
                (None, BlockType::Empty)
            };
            Block {
                block_type,
                len,
                id,
            }
        })
        .collect::<Vec<_>>();
    trim_disk(&mut disk);

    // show_layout(&disk);
    while let Some(idx) = find_empty(&disk) {
        let id = {
            let last_block = disk.last_mut().unwrap();
            last_block.len -= 1;
            last_block.id.clone()
        };

        disk.get_mut(idx).unwrap().len -= 1;

        disk.insert(
            idx,
            Block {
                len: 1,
                block_type: BlockType::File,
                id,
            },
        );
        trim_disk(&mut disk);
        // show_layout(&disk);
    }

    let mut check_sum = 0;
    let mut count = 0;
    for block in disk {
        for _ in 0..block.len {
            check_sum += block.id.unwrap() * count;
            count += 1;
        }
    }
    println!("{check_sum}");
}

fn find_empty(disk: &Vec<Block>) -> Option<usize> {
    for i in 0..disk.len() {
        if disk[i].block_type == BlockType::Empty {
            return Some(i);
        }
    }
    None
}

fn trim_disk(disk: &mut Vec<Block>) {
    disk.retain(|block| block.len > 0);

    while disk.last().is_some()
        && (disk.last().unwrap().block_type == BlockType::Empty || disk.last().unwrap().len == 0)
    {
        disk.pop();
    }
}

fn show_layout(disk: &Vec<Block>) {
    for block in disk {
        let block_repr = if block.block_type == BlockType::File {
            format!("{}", block.id.unwrap())
        } else {
            ".".to_string()
        }
        .repeat(block.len as usize);

        if block.len == 0 {
            print!("#");
        } else {
            print!("{block_repr}");
        }
    }
    println!(" ({} blk)", disk.len())
}
