use aoc_runner_derive::{aoc, aoc_generator};
use std::mem::{discriminant};
use std::collections::HashSet;

#[derive(Clone)]
enum GridMember {
    X,
    M,
    A,
    S,
}

fn grid_member_from_char(c: char) -> Option<GridMember> {
    match c {
        'X' => Some(GridMember::X),
        'M' => Some(GridMember::M),
        'A' => Some(GridMember::A),
        'S' => Some(GridMember::S),
        _ => None,
    }
}

// fn is_member(sample: Option<GridMember>, test: Option<GridMember>) -> bool {
//     match sample {
//         test => true,
//         _ => false,
//     }
// }

fn bound_vector(input: usize, length: usize, input_max: usize, bounded: bool, negative: bool) -> Vec<usize> {
    let l: usize = length - 1;
    if negative {
        if input < l {
            if bounded {
                vec![]
            } else {
                // should be safe to convert to isize since we know the bounds
                ((input as isize)-(l as isize)..(input as isize)+1).rev().map(|i| wrap_vector(i, input_max)).collect::<Vec<usize>>()
            }
        } else {
            (input-l..input+1).rev().collect::<Vec<usize>>()
        }
    } else {
        if input + length > input_max {
            if bounded {
                vec![]
            } else {
                // should be safe to convert to isize since we know the bounds
                (input as isize..(input+length) as isize).map(|i| wrap_vector(i, input_max)).collect::<Vec<usize>>()
            }
        } else {
            (input..input+length).collect::<Vec<usize>>()
        }
    }
}

fn wrap_vector(i: isize, i_max: usize) -> usize {
    if i < 0 {
        (i + i_max as isize) as usize
    } else if i >= i_max as isize {
        (i - i_max as isize) as usize
    } else {
        i as usize
    }
}

struct SearchGrid {
    input: String,
    grid: Vec<Vec<Option<GridMember>>>,
    x_max: usize,
    y_max: usize,
}

impl SearchGrid {
    fn from_input(input: &str) -> Self {
        let grid: Vec<Vec<Option<GridMember>>> = input
        .lines()
        // .iter()
        .map(
            |l| l
            .chars()
            // .iter()
            .map(|c| grid_member_from_char(c))
            .collect::<Vec<Option<GridMember>>>()
        )
        .collect();

        // check lengths
        let lengths: Vec<usize> = grid
        .iter()
        .map(|v| v.len())
        .collect();

        let unique_lengths: HashSet<usize> = lengths.to_owned().into_iter().collect();

        if unique_lengths.len() != 1 {
            panic!("Input of inconsistent length, not a grid! Lengths: {:?}", lengths)
        }

        let x_max: usize = lengths[0];
        let y_max: usize = grid.len();

        // Debug
        // println!("x_max: {}, y_max: {}", x_max, y_max);

        SearchGrid { input: input.to_string(), grid, x_max, y_max }
    }

    fn generate_star_coordinates(&self, X_coord: (usize, usize), bounded: bool) -> Vec<Vec<(usize, usize)>> {
        let x: usize = X_coord.0;
        let y: usize = X_coord.1;

        let left: Vec<usize> = bound_vector(x, 4, self.x_max, bounded, true);
        let right: Vec<usize> = bound_vector(x, 4, self.x_max, bounded, false);
        let up: Vec<usize> = bound_vector(y, 4, self.y_max, bounded, true);
        let down: Vec<usize> = bound_vector(y, 4, self.y_max, bounded, false);
        
        let xs_vec: Vec<Vec<usize>> = vec![left, vec![x; 4], right].into_iter().filter(|v| v.len() > 0).collect();
        let ys_vec: Vec<Vec<usize>> = vec![up, vec![y; 4], down].into_iter().filter(|v| v.len() > 0).collect();

        let result: Vec<Vec<(usize, usize)>> = xs_vec
        .into_iter()
        .map(
            |xs| ys_vec.clone()
            .into_iter()
            .map(
                |ys| if !((xs == vec![x; 4]) && (ys == vec![y; 4])) {
                    xs.clone().into_iter().zip(ys).collect::<Vec<(usize, usize)>>()
                } else { vec![] }
            )
            .filter(|v| v.len() > 0)
            .collect::<Vec<Vec<(usize, usize)>>>()
        )
        .flatten()
        .collect();

        result
    }

    fn search_neighbours(&self, path: Vec<(usize, usize)>, target: &GridMember) -> Vec<Vec<(usize, usize)>> {
        let x: usize = path.last().unwrap().0;
        let y: usize = path.last().unwrap().1;

        // combination inputs
        let x_cs: Vec<usize> = if x == 0 {
            vec![0, 1]
        } else if x == self.x_max-1 {
            vec![x-1, x]
        } else {
            vec![x-1, x, x+1]
        };
        let y_cs: Vec<usize> = if y == 0 {
            vec![0,1]
        } else if y == self.y_max-1 {
            vec![y-1, y]
        } else {
            vec![y-1, y, y+1]
        };


        // generate indices
        let mut cs: Vec<(usize, usize)> = vec![];
        for a in &x_cs {
            for b in &y_cs {
                cs.push((*a,*b));
            }
        }
        
        let index = cs.iter().position(|(a,b)| *a == x && *b == y).unwrap();
        cs.remove(index);

        // Debug
        // println!("cs ({},{}): {:?}", x, y, cs);
        
        let filtered: Vec<(usize, usize)> = (
            *cs
            .to_owned()
            .into_iter()
            .filter(
                |(x,y)| match &self.grid[*y][*x]{
                    Some(t) => discriminant(t) == discriminant(target),
                    _ => false,
                }
            )
            .collect::<Vec<(usize, usize)>>()
        ).to_vec();
        let result: Vec<Vec<(usize, usize)>> = filtered.to_owned().iter().map(|i| [path.clone(), vec![i.to_owned()]].concat()).collect();

        result
    }

    fn find_xmas(&self) -> Vec<Vec<(usize, usize)>> {
        // Find all coordinates of X
        let grid_coords: Vec<(usize, usize)> = (0..self.x_max)
        .map(|x| (0..self.y_max).map(|y| (x, y)).collect::<Vec<(usize, usize)>>())
        .flatten()
        .to_owned()
        .collect();
        
        let X_coords: Vec<(usize, usize)> = grid_coords
        // .to_owned()
        .into_iter()
        .filter(
            |(x,y)| match &self.grid[*y][*x].clone() {
                Some(m) => discriminant(m) == discriminant(&GridMember::X),
                _ => false,
            }
        )
        .collect();

        // Debug
        // for X in &X_coords {
        //     println!("{:?}: {}", X, self.input.replace('\n', "").chars().nth(X.1*10 + X.0).unwrap())
        // }
        // println!("Total Xs: {}", X_coords.len());

        let search_space: Vec<Vec<(usize, usize)>> = X_coords.into_iter().map(|c| self.generate_star_coordinates(c, true)).flatten().collect();

        // Debug
        // for path in &search_space {
        //     println!("Search path: {:?}", path);
        // }
        // println!("Search space size: {}", search_space.len());

        let result: Vec<Vec<(usize, usize)>> = search_space.into_iter().filter(|v| self.check_xmas(v.to_owned())).collect();
        
        result
    }

    fn find_xmas_snake(&self) -> Vec<Vec<(usize, usize)>> {
        // Find all coordinates of XMAS

        // Find all coordinates of X
        let grid_coords: Vec<(usize, usize)> = (0..self.x_max)
        .map(|x| (0..self.y_max).map(|y| (x, y)).collect::<Vec<(usize, usize)>>())
        .flatten()
        .to_owned()
        .collect();
        
        let X_coords: Vec<Vec<(usize, usize)>> = grid_coords
        .into_iter()
        .filter(
            |(x,y)| match self.grid[*y][*x] {
                Some(GridMember::X) => true,
                _ => false,
            }
        )
        .map(|x| vec![x])
        .collect();

        // Find all coordinates of M that are neighbours of X.
        // This should necessarily be a smaller search space than the above.
        let M_coords: Vec<Vec<(usize, usize)>> = X_coords.iter().map(|p| self.search_neighbours(p.to_owned(), &GridMember::M)).flatten().collect();
        println!("m_coords ({}): {:?}", M_coords.len(), M_coords);

        // Find all coordinates of A that are neighbours of M, that are neighbours of X.
        // This should necessarily be a smaller search space than the above.
        let A_coords: Vec<Vec<(usize, usize)>> = M_coords.iter().map(|p| self.search_neighbours(p.to_owned(), &GridMember::A)).flatten().collect();
        println!("a_coords ({}): {:?}", A_coords.len(), A_coords);

        // Find all coordinates of S that are neighbours of A, that are neighbours of M, that are neighbours of X.
        // This should necessarily be a smaller search space than the above.
        let S_coords: Vec<Vec<(usize, usize)>> = M_coords.iter().map(|p| self.search_neighbours(p.to_owned(), &GridMember::S)).flatten().collect();

        S_coords
    }

    fn find_x_mas(&self) -> Vec<(usize, usize)> {
        // Find all coordinates of A
        let A_coords: Vec<(usize, usize)> = (1..self.x_max-1)
        .map(|x| (1..self.y_max-1).map(|y| (x, y)).collect::<Vec<(usize, usize)>>())
        .flatten()
        .to_owned()
        .collect::<Vec<(usize, usize)>>()
        .into_iter()
        .filter(
            |(x,y)| match &self.grid[*y][*x].clone() {
                Some(m) => discriminant(m) == discriminant(&GridMember::A),
                _ => false,
            }
        )
        .collect();

        let result: Vec<(usize, usize)> = A_coords.into_iter().filter(|c| self.check_corners(*c)).collect();

        // Debug
        // println!("X-MAS: {:?}", result);
        
        result
    }

    fn count_x_mas(&self) -> usize {
        self.find_x_mas().len()
    }

    fn count_xmas(&self) -> usize {
        let paths: Vec<Vec<(usize, usize)>> = self.find_xmas();
        // Debug
        // println!("paths: {:?}", paths);
        // for path in &paths {
        //     println!(
        //         "{:?}: {}{}{}{}",
        //         path,
        //         self.input.replace('\n', "").chars().nth(path[0].1*10 + path[0].0).unwrap(),
        //         self.input.replace('\n', "").chars().nth(path[1].1*10 + path[1].0).unwrap(),
        //         self.input.replace('\n', "").chars().nth(path[2].1*10 + path[2].0).unwrap(),
        //         self.input.replace('\n', "").chars().nth(path[3].1*10 + path[3].0).unwrap(),
        //     );
        // }
        paths.len()
    }

    fn check_corners(&self, input: (usize, usize)) -> bool {
        let x: usize = input.0;
        let y: usize = input.1;

        let corners: Vec<(usize, usize)> = vec![(x-1, y-1), (x+1, y-1), (x-1, y+1), (x+1, y+1),];

        let ss: Vec<(usize, usize)> = corners
        .clone()
        .into_iter()
        .filter(
            |c| match &self.grid[c.1][c.0] {
                Some(m) => discriminant(m) == discriminant(&GridMember::S),
                None => false,
            }
        )
        .collect();

        let ms: Vec<(usize, usize)> = corners
        .clone()
        .into_iter()
        .filter(
            |c| match &self.grid[c.1][c.0] {
                Some(m) => discriminant(m) == discriminant(&GridMember::M),
                None => false,
            }
        )
        .collect();

        let ss_x: HashSet<usize> = ss.clone().into_iter().map(|c| c.0).collect();
        let ss_y: HashSet<usize> = ss.clone().into_iter().map(|c| c.1).collect();
        let ms_x: HashSet<usize> = ms.clone().into_iter().map(|c| c.0).collect();
        let ms_y: HashSet<usize> = ms.clone().into_iter().map(|c| c.1).collect();

        (ss.len() == 2) && (ms.len() == 2) && ((ss_x.len() + ss_y.len()) == 3) && ((ms_x.len() + ms_y.len()) == 3)
    }
    
    fn check_xmas(&self, input: Vec<(usize, usize)>) -> bool {
        // Takes vector of input path, returns if XMAS
        if input.len() == 4 {
            let enums: Vec<GridMember> = vec![GridMember::X, GridMember::M, GridMember::A, GridMember::S];
            input.into_iter().zip(enums).all(|(i,e)| match &self.grid[i.1][i.0].clone() {
                Some(m) => discriminant(m) == discriminant(&e),
                None => false
            })
        } else {
            false
        }
    }
}

fn generate_sample() -> SearchGrid {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    part1_input(input)
}

#[aoc_generator(day4)]
fn part1_input(input: &str) -> SearchGrid {
    SearchGrid::from_input(input)
}

#[aoc(day4, part1, mine)]
fn part1(input: &SearchGrid) -> usize {
    let test_grid: SearchGrid = generate_sample();
    let test_count: usize = test_grid.count_xmas();
    println!("Test count: {}", test_count);

    let count: usize = input.count_xmas();
    count
}

#[aoc(day4, part2, mine)]
fn part2(input: &SearchGrid) -> usize {
    let test_grid: SearchGrid = generate_sample();
    let test_count: usize = test_grid.count_x_mas();
    println!("Test count: {}", test_count);

    let count: usize = input.count_x_mas();
    count
}