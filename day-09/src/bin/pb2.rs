#[derive(Debug, Clone)]
struct Block {
    pub id: usize,
    pub len: usize,
    pub start: usize,
}

fn main() {
    let src = include_str!("src2.txt");
    // let src = "2133133121414131402";
    // let src = "19234512365432196847485";

    let mut descriptor = src
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c as usize - '0' as usize;
            let id = if i % 2 == 0 { i / 2 } else { usize::MAX };
            Block { len, id, start: 0 }
        })
        .collect::<Vec<_>>();

    let mut disk = Vec::new();
    let mut count = 0;
    for block in descriptor.iter_mut() {
        block.start = count;
        count += block.len as usize;
        for _ in 0..block.len {
            disk.push(block.id);
        }
    }

    show_layout(&disk);
    fall_blocks(&mut disk, &mut descriptor);
    show_layout(&disk);
    let result = disk.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + i * if *v == usize::MAX { 0 } else { *v }
    });
    println!("{result}");
}

fn show_layout(disk: &Vec<usize>) {
    for block in disk[0..60].iter() {
        if *block < usize::MAX {
            print!("{block:x}");
        } else {
            print!(".");
        }
    }
    println!()
}

fn fall_blocks(disk: &mut Vec<usize>, descriptor: &mut Vec<Block>) {
    let mut file_blocks = descriptor
        .iter()
        .rev()
        .filter(|block| block.id < usize::MAX)
        .map(Block::clone)
        .collect::<Vec<Block>>();

    let mut empty_blocks = descriptor
        .iter()
        .filter(|block| block.id == usize::MAX)
        .map(Block::clone)
        .collect::<Vec<Block>>();

    empty_blocks.sort_by_key(|block| block.start);

    for i in 0..file_blocks.len() {
        show_layout(disk);
        // println!("Try move: 0x{:x} {}", file_blocks[i].id, file_blocks[i].len);
        // println!("{:?}", empty_blocks.iter().map(|blk| (blk.start, blk.len)).collect::<Vec<_>>());
        let file_block = &mut file_blocks[i];
        for j in 0..empty_blocks.len() {
            let empty_block = &mut empty_blocks[j];
            if empty_block.len >= file_block.len && empty_block.start < file_block.start {
                // println!("Move to {}", empty_block.start);
                for i in 0..file_block.len {
                    disk.swap(empty_block.start + i, file_block.start + i);
                }
                file_block.start = empty_block.start;
                empty_block.start += file_block.len;
                empty_block.len -= file_block.len;

                empty_blocks.sort_by_key(|block| block.start);

                break;
            }
        }
    }
}
