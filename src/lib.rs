//! A function to convert ASCII-art representations of letters generated by Advent of Code 
//! puzzles into a String containing those letters.

use std::{collections::HashMap, env, fs};
pub mod scannable;
pub use crate::scannable::Scannable;

/// Takes an image containing Advent of Code's ASCII-art letter representations and converts 
/// it to a standard String.
/// 
/// `image` is a `Scannable,` which is a marker trait that is implemented for the following
/// types:
/// * `&str`
/// * `(&Vec<bool>, usize)`, a tuple consisting of a Vec of bools and the width of a line.
/// * `(&Vec<bool>, usize)`, a tuple consisting of a Vec of chars and the width of a line.
/// * `&Vec<Vec<bool>>`, a Vec of a Vec of bools.
/// * `&Vec<Vec<char>>`, a Vec of a Vec of chars.
/// 
/// For &str and the char-based Vecs, '#' is considered part of a letter and all other chars 
/// are considered blank space. For the bool-based Vecs, `true` is considered part of a 
/// letter and `false` is considered blank space.
/// 
/// # Example
/// 
/// ```
/// let image = r"
/// .##..###...##.
/// ##..#.#..#.#..#
/// ##..#.###..#...
/// #####.#..#.#...
/// ##..#.#..#.#..#
/// ##..#.###...##.
///     ";
/// 
/// let s = ocr(image);
/// assert_eq!(s, "ABC");
/// ```
pub fn ocr<T: Scannable>(image: T) -> Option<String> {
    let image = image.normalize();
    let image = image.trim();
    let ids = map_to_id(image)?;
    let letter_map = get_letter_map();
    let ocr = ids.iter()
        .map(|id| letter_map.get(id).unwrap_or(&'?'))
        .collect();
    Some(ocr)
}

fn map_to_id(image: &str) -> Option<Vec<u64>> {
    let width = image.find('\n')?;
    let height = image.len() / width;
    let image = image.as_bytes();
    
    let mut id = 0u64;
    let mut letter_width = 0usize;

    let mut ids = Vec::new();

    for x in 0..width {
        let col: Vec<bool> = (0..height)
            .map(|y| image[x + y * (width + 1)] == b'#')
            .collect();
        if col.iter().all(|&b| !b) {
            if id != 0 { ids.push(id); }
            id = 0;
            letter_width = 0;
        } else {
            if height == 6 && letter_width == 5 {
                ids.push(id);
                id = 0;
                letter_width = 0;
            }
            id = col.iter()
                .fold(id, |acc, &b| (acc << 1) + if b { 1 } else { 0 });
            letter_width += 1;
        }
    }
    if id != 0 { ids.push(id) };
    Some(ids)
}

fn get_letter_map() -> HashMap<u64, char> {
    let font6 = fs::read_to_string("res/font6.txt").unwrap();
    let (letters6, letter_forms6) = font6
        .split_once("\n\n")
        .unwrap();

    let font10 = fs::read_to_string("res/font10.txt").unwrap();
    let (letters10, letter_forms10) = font10
        .split_once("\n\n")
        .unwrap();

    let mut letter_map = HashMap::new();

    populate_letter_map(&mut letter_map, letter_forms6, letters6);
    populate_letter_map(&mut letter_map, letter_forms10, letters10);

    letter_map
}

fn populate_letter_map(
    letter_map: &mut HashMap<u64, char>, 
    letter_forms: &str, 
    letters: &str
) {
    map_to_id(letter_forms.trim())
        .unwrap()
        .iter()
        .zip(letters.chars()) 
        .for_each(|(&id, c)| {
            letter_map.insert(id, c);
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ocr_test<T: Scannable>(output: &str, letter_forms: T) -> bool {
        Some(output.to_string()) == ocr(letter_forms)
    }


    #[test]
    fn size6() {
        let output = "ABCEFGHIJKLOPRSUYZ";
        let letter_forms = r"
.##..###...##..####.####..##..#..#.###...##.#..#.#.....##..###..###...###.#..#.#...#.####
#..#.#..#.#..#.#....#....#..#.#..#..#.....#.#.#..#....#..#.#..#.#..#.#....#..#.#...#....#
#..#.###..#....###..###..#....####..#.....#.##...#....#..#.#..#.#..#.#....#..#..#.#....#.
####.#..#.#....#....#....#.##.#..#..#.....#.#.#..#....#..#.###..###...##..#..#...#....#..
#..#.#..#.#..#.#....#....#..#.#..#..#..#..#.#.#..#....#..#.#....#.#.....#.#..#...#...#...
#..#.###...##..####.#.....###.#..#.###..##..#..#.####..##..#....#..#.###...##....#...####        
        ";
        assert!(ocr_test(output, letter_forms));
    }

    #[test]
    fn size10() {
        let output = "ABCEFGHJKLNPRXZ";
        let letter_forms = r"
..##...#####...####..######.######..####..#....#....###.#....#.#......#....#.#####..#####..#....#.######
.#..#..#....#.#....#.#......#......#....#.#....#.....#..#...#..#......##...#.#....#.#....#.#....#......#
#....#.#....#.#......#......#......#......#....#.....#..#..#...#......##...#.#....#.#....#..#..#.......#
#....#.#....#.#......#......#......#......#....#.....#..#.#....#......#.#..#.#....#.#....#..#..#......#.
#....#.#####..#......#####..#####..#......######.....#..##.....#......#.#..#.#####..#####....##......#..
######.#....#.#......#......#......#..###.#....#.....#..##.....#......#..#.#.#......#..#.....##.....#...
#....#.#....#.#......#......#......#....#.#....#.....#..#.#....#......#..#.#.#......#...#...#..#...#....
#....#.#....#.#......#......#......#....#.#....#.#...#..#..#...#......#...##.#......#...#...#..#..#.....
#....#.#....#.#....#.#......#......#...##.#....#.#...#..#...#..#......#...##.#......#....#.#....#.#.....
#....#.#####...####..######.#.......###.#.#....#..###...#....#.######.#....#.#......#....#.#....#.######
        ";
        assert!(ocr_test(output, letter_forms));
    }

    #[test]
    fn y_bug() {
        let output = "LGYHB";
        let letter_forms = r"
#.....##..#...##..#.###..
#....#..#.#...##..#.#..#.
#....#.....#.#.####.###..
#....#.##...#..#..#.#..#.
#....#..#...#..#..#.#..#.
####..###...#..#..#.###..
        ";
        assert!(ocr_test(output, letter_forms));
    }

    #[test]
    fn bool_vec() {
        let output = "LGYHB";
        let letter_forms = r"
#.....##..#...##..#.###..
#....#..#.#...##..#.#..#.
#....#.....#.#.####.###..
#....#.##...#..#..#.#..#.
#....#..#...#..#..#.#..#.
####..###...#..#..#.###..
        ".trim();
        let width = letter_forms.find('\n').unwrap();
        let bools: Vec<_> = letter_forms.as_bytes().iter()
            .filter(|&&c| c != b'\n')
            .map(|&c| c == b'#')
            .collect();
        assert!(ocr_test(output, (&bools, width)));
    }

    #[test]
    fn char_vec() {
        let output = "ABCEFGHJKLNPRXZ";
        let letter_forms = r"
..##...#####...####..######.######..####..#....#....###.#....#.#......#....#.#####..#####..#....#.######
.#..#..#....#.#....#.#......#......#....#.#....#.....#..#...#..#......##...#.#....#.#....#.#....#......#
#....#.#....#.#......#......#......#......#....#.....#..#..#...#......##...#.#....#.#....#..#..#.......#
#....#.#....#.#......#......#......#......#....#.....#..#.#....#......#.#..#.#....#.#....#..#..#......#.
#....#.#####..#......#####..#####..#......######.....#..##.....#......#.#..#.#####..#####....##......#..
######.#....#.#......#......#......#..###.#....#.....#..##.....#......#..#.#.#......#..#.....##.....#...
#....#.#....#.#......#......#......#....#.#....#.....#..#.#....#......#..#.#.#......#...#...#..#...#....
#....#.#....#.#......#......#......#....#.#....#.#...#..#..#...#......#...##.#......#...#...#..#..#.....
#....#.#....#.#....#.#......#......#...##.#....#.#...#..#...#..#......#...##.#......#....#.#....#.#.....
#....#.#####...####..######.#.......###.#.#....#..###...#....#.######.#....#.#......#....#.#....#.######
        ".trim();
        let width = letter_forms.find('\n').unwrap();
        let chars: Vec<_> = letter_forms.chars()
            .filter(|&c| c != '\n')
            .collect();
        assert!(ocr_test(output, (&chars, width)));
    }

    #[test]
    fn vec_vec_char() {
        let output = "ABCEFGHJKLNPRXZ";
        let letter_forms = r"
..##...#####...####..######.######..####..#....#....###.#....#.#......#....#.#####..#####..#....#.######
.#..#..#....#.#....#.#......#......#....#.#....#.....#..#...#..#......##...#.#....#.#....#.#....#......#
#....#.#....#.#......#......#......#......#....#.....#..#..#...#......##...#.#....#.#....#..#..#.......#
#....#.#....#.#......#......#......#......#....#.....#..#.#....#......#.#..#.#....#.#....#..#..#......#.
#....#.#####..#......#####..#####..#......######.....#..##.....#......#.#..#.#####..#####....##......#..
######.#....#.#......#......#......#..###.#....#.....#..##.....#......#..#.#.#......#..#.....##.....#...
#....#.#....#.#......#......#......#....#.#....#.....#..#.#....#......#..#.#.#......#...#...#..#...#....
#....#.#....#.#......#......#......#....#.#....#.#...#..#..#...#......#...##.#......#...#...#..#..#.....
#....#.#....#.#....#.#......#......#...##.#....#.#...#..#...#..#......#...##.#......#....#.#....#.#.....
#....#.#####...####..######.#.......###.#.#....#..###...#....#.######.#....#.#......#....#.#....#.######
        ".trim();
        let chars: Vec<Vec<char>> = letter_forms.lines()
            .map(|line| line.chars().collect())
            .collect();
        assert!(ocr_test(output, &chars));
    }

    #[test]
    fn vec_vec_bool() {
        let output = "ABCEFGHJKLNPRXZ";
        let letter_forms = r"
..##...#####...####..######.######..####..#....#....###.#....#.#......#....#.#####..#####..#....#.######
.#..#..#....#.#....#.#......#......#....#.#....#.....#..#...#..#......##...#.#....#.#....#.#....#......#
#....#.#....#.#......#......#......#......#....#.....#..#..#...#......##...#.#....#.#....#..#..#.......#
#....#.#....#.#......#......#......#......#....#.....#..#.#....#......#.#..#.#....#.#....#..#..#......#.
#....#.#####..#......#####..#####..#......######.....#..##.....#......#.#..#.#####..#####....##......#..
######.#....#.#......#......#......#..###.#....#.....#..##.....#......#..#.#.#......#..#.....##.....#...
#....#.#....#.#......#......#......#....#.#....#.....#..#.#....#......#..#.#.#......#...#...#..#...#....
#....#.#....#.#......#......#......#....#.#....#.#...#..#..#...#......#...##.#......#...#...#..#..#.....
#....#.#....#.#....#.#......#......#...##.#....#.#...#..#...#..#......#...##.#......#....#.#....#.#.....
#....#.#####...####..######.#.......###.#.#....#..###...#....#.######.#....#.#......#....#.#....#.######
        ".trim();
        let bools: Vec<Vec<bool>> = letter_forms.lines()
            .map(|line| { 
                line.chars().map(|c| c == '#').collect()
            }).collect();
        assert!(ocr_test(output, &bools));
    }

}
