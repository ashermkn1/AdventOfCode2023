#[derive(Debug)]
struct Landscape {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Landscape> {
    input
        .split("\n\n")
        .map(|block| {
            let mut rows = vec![0u32; block.lines().count()];
            let mut cols = vec![0u32; block.find('\n').unwrap()];

            for (row_ind, row) in block.lines().enumerate() {
                for (col_ind, c) in row.char_indices() {
                    let val = u32::from(c == '#');
                    cols[col_ind] = (cols[col_ind] << 1) + val;
                    rows[row_ind] = (rows[row_ind] << 1) + val;
                }
            }
            Landscape { rows, cols }
        })
        .collect()
}

fn reflect(image: &[u32]) -> usize {
    'outer: for i in 1..image.len() {
        let mut low = i;
        let mut high = i - 1;
        while low > 0 && high < image.len() - 1 {
            low -= 1;
            high += 1;
            if image[low] != image[high] {
                continue 'outer;
            }
        }
        return i;
    }
    0
}

fn one_bit_diff(a: u32, b: u32) -> bool {
    let x = a ^ b;
    x & (x - 1) == 0
}
fn reflect_smudge(image: &[u32]) -> usize {
    'outer: for i in 1..image.len() {
        let mut found_smudge = false;
        let mut low = i;
        let mut high = i - 1;
        while low > 0 && high < image.len() - 1 {
            low -= 1;
            high += 1;
            if image[low] == image[high] {
                continue;
            }

            if !found_smudge && one_bit_diff(image[low], image[high]) {
                found_smudge = true;
            } else {
                continue 'outer;
            }
        }
        if found_smudge {
            return i;
        }
    }
    0
}
fn score(landscape: &Landscape) -> usize {
    let vert = reflect(&landscape.cols);
    if vert > 0 {
        vert
    } else {
        reflect(&landscape.rows) * 100
    }
}

fn smudge_score(landscape: &Landscape) -> usize {
    let vert = reflect_smudge(&landscape.cols);
    if vert > 0 {
        vert
    } else {
        reflect_smudge(&landscape.rows) * 100
    }
}
#[aoc(day13, part1)]
fn part1(input: &[Landscape]) -> usize {
    input.iter().map(score).sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Landscape]) -> usize {
    input.iter().map(smudge_score).sum()
}
