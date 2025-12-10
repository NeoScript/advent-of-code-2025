use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    part_one();
}

fn count_accessible_rolls(m: Vec<Vec<char>>) -> usize {
    m.into_iter().flatten().filter(|c| *c == 'x').count()
}

fn find_possible_rolls(m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut matrix = m.clone();
    let rows = matrix.len();
    let cols = matrix.first().unwrap().len();

    let offsets: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for r in 0..rows {
        for c in 0..cols {
            let curr = matrix[r][c];
            println!("Currently looking at maxtrix[{r}][{c}] -> {}", curr);
            if curr == '.' {
                continue;
            }

            let mut neighbor_count = 0;
            for (dr, dc) in &offsets {
                let y: isize = r as isize + dr;
                let x: isize = c as isize + dc;

                match matrix.get(y as usize) {
                    Some(row) => match row.get(x as usize) {
                        Some(element) => {
                            println!("checking neighbor [{}][{}] -> {}", y, x, element);
                            match *element {
                                'x' | '@' => {
                                    neighbor_count += 1;
                                }
                                _ => continue,
                            }
                        }
                        None => println!("skipping neighbor [{}][{}] -> col out of bounds", y, x),
                    },
                    None => println!("skipping neighbor [{}][{}] -> row out of bounds", y, x),
                }
            }

            println!("neighbor_count: {}", neighbor_count);
            if neighbor_count < 4 {
                matrix[r][c] = 'x';
            }
        }
    }

    matrix
}

fn part_one() {
    let path = Path::new("input.txt");
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            line.chars().collect()
        })
        .collect();

    let result = find_possible_rolls(matrix);
    result.iter().for_each(|l| {
        l.iter().for_each(|c| {
            print!("{}", c);
        });
        println!();
    });

    let count = count_accessible_rolls(result);
    println!("part one count: {}", count);
}
