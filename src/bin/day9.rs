use std::mem;

use aoc24::*;

fn main() {
    let binding = read_input(9);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    //println!("Part 2: {}", part2("2333133121414131402"));
    println!("Part 2: {}", part2(&input));
}

// TODO: Make soln more efficient/idiomatic
fn part1(input: &str) -> String {
    let segments = parse_row_major::<usize>(input, "");
    let segments = &segments[0];

    let mut disk: Vec<Option<usize>> = Vec::new();
    segments.iter().enumerate().for_each(|(i, x)| {
        if i % 2 == 0 {
            for _ in 0..*x {
                disk.push(Some(i / 2))
            }
        } else {
            for _ in 0..*x {
                disk.push(None)
            }
        }
    });

    let mut sum = 0;
    {
        let mut i = 0;
        while i < disk.len() {
            if disk[i].is_some() {
                sum += i * disk[i].unwrap();
            } else {
                while disk[disk.len() - 1].is_none() {
                    disk.pop();
                }
                let popped_val = disk.pop().unwrap();
                disk[i] = popped_val;
                sum += i * popped_val.unwrap();
            }
            i += 1
        }
    }

    return sum.to_string();
}

fn part2(input: &str) -> String {
    let segments = parse_row_major::<usize>(input, "");
    let segments = &segments[0];

    let mut disk: Vec<DiskBlock> = Vec::new();

    segments.iter().enumerate().fold(0, |idx, (i, x)| {
        disk.push(DiskBlock::new(*x, i / 2, idx, i % 2 == 0));
        idx + x
    });
    //print_blocks(&disk);
    //for block in &disk {
    //    println!("{}", block.display());
    //}

    let defrag = defrag2(disk);

    //print_blocks(&defrag);
    //
    //for block in &defrag {
    //    println!("{} {}", block.display(), block.checksum());
    //}

    return defrag
        .iter()
        .fold(0, |acc, b| acc + b.checksum())
        .to_string();
}

fn defrag2(disk: Vec<DiskBlock>) -> Vec<DiskBlock> {
    let mut defrag = disk.to_vec();
    // in order to do this asymptotically fastest, we'd probably want a bst of some sorts
    // We want to query the leftmost free block with a size greater than our block. I don't know
    // how to turn this into a comparator function though...
    for i in (0..disk.len()).rev() {
        if !disk[i].full {
            continue;
        }
        let mut j = 1;
        while defrag[j].idx < disk[i].idx {
            // shift block to preceding empty block if no other options.
            //if defrag[j].id == disk[i].id - 1 && !defrag[j].full && defrag[j].size > 0 {
            //    let full_block = defrag[j + 1].clone();
            //    println!("{}", full_block.display());
            //    defrag[j + 1].size = defrag[j].size;
            //    defrag[j + 1].idx = defrag[j].size + defrag[j].idx;
            //    defrag[j + 1].full = false;
            //    defrag[j].size = full_block.size;
            //    defrag[j].id = full_block.id;
            //    defrag[j].full = true;
            //    break;
            //}

            if !defrag[j].full && defrag[j].size >= disk[i].size {
                shift_block(&mut defrag, j, &disk, i);
                //println!("remove {}", to_remove_idx);

                //print_blocks(&defrag);

                break;
            }
            j += 1;
        }
    }
    defrag
}

fn shift_block(defrag: &mut Vec<DiskBlock>, j: usize, disk: &Vec<DiskBlock>, i: usize) {
    defrag.splice(
        j..j,
        [DiskBlock::new(
            disk[i].size,
            disk[i].id,
            defrag[j].idx,
            true,
        )],
    );
    defrag[j + 1].size -= disk[i].size;
    defrag[j + 1].idx += disk[i].size;

    let to_remove_idx = defrag
        .iter()
        .skip(j + 1)
        .position(|x| x.id == disk[i].id)
        .unwrap()
        + (j + 1);
    assert_ne!(to_remove_idx, j);
    defrag[to_remove_idx].full = false;
}

fn print_blocks(disk: &Vec<DiskBlock>) {
    block_repr(disk);
    println!();
}

fn block_repr(disk: &Vec<DiskBlock>) -> String {
    let mut ret = String::new();
    for block in disk {
        if block.full {
            for _ in 0..block.size {
                ret.push_str(&block.id.to_string());
            }
        } else {
            for _ in 0..block.size {
                ret.push_str(".");
            }
        }
    }
    ret
}

fn parse_segments(input: &str) -> Vec<DiskBlock> {
    let segments = parse_row_major::<usize>(input, "");
    let segments = &segments[0];

    let mut disk: Vec<DiskBlock> = Vec::new();

    segments.iter().enumerate().fold(0, |idx, (i, x)| {
        disk.push(DiskBlock::new(*x, i / 2, idx, i % 2 == 0));
        idx + x
    });
    disk
}

#[derive(Clone)]
struct DiskBlock {
    size: usize,
    id: usize,
    idx: usize,
    full: bool,
}

impl DiskBlock {
    fn new(size: usize, id: usize, idx: usize, full: bool) -> Self {
        DiskBlock {
            size,
            id,
            idx,
            full,
        }
    }

    fn checksum(&self) -> usize {
        if self.full {
            return self.idx * self.id * self.size + self.id * self.size * (self.size - 1) / 2;
        } else {
            return 0;
        }
    }

    fn display(&self) -> String {
        format!(
            "size:{}, id:{}, idx:{}, full:{}",
            self.size, self.id, self.idx, self.full
        )
    }
}

fn disk_checksum(disk: &Vec<usize>) -> usize {
    disk.iter()
        .enumerate()
        .fold(0, |acc, (i, val)| acc + i * val)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "1928");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "2858");
    }

    #[test]
    fn test_block_repr() {
        assert_eq!(block_repr(&defrag2(parse_segments("111"))).as_str(), "01.");
    }

    #[test]
    fn test_edge() {
        assert_eq!(
            block_repr(&defrag2(parse_segments("1313165"))).as_str(),
            "021......33333......"
        );
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(
            part2("673253833464635054191677274350925861527651788483"),
            "149706"
        );
    }

    #[test]
    fn test_part2_ex2() {
        assert_eq!(part2("23222120202525282820202020272722212121"), "7705");
    }

    #[test]
    fn test_part2_ex3() {
        assert_eq!(
            part2("22222228282828222222282829212324252627282920"),
            "9447"
        );
    }
}
