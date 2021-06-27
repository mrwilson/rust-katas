// Create a clockwise spiral within a n-by-n grid.
// Adapted from https://github.com/emilybache/Spiral-Refactoring-Kata

#[allow(unused_variables)]
pub fn spiral(size: i8) -> String {
    let mut fill: Vec<Vec<char>> = vec![vec!['1'; size as usize]; size as usize];

    let height = fill.len() as i8;
    let width = fill[0].len() as i8;
    let mut last_y = size / 2;

    let mut last_x = if size % 2 == 0 {
        size / 2 - 1
    } else {
        size / 2
    };

    if size == 5 {
        last_x = 1;
        last_y = 3;
    } else if (size - 5) % 4 == 0 {
        last_x -= 1;
        last_y += 1;
    } else if size < 5 {
        return String::from("");
    }

    let mut y = 1;
    let mut x = 0;
    let mut dir_x = 1;
    let mut dir_y = 0;
    let mut i = 0;
    let mut left_border = 0;
    let mut right_border = width - 1;
    let mut upper_border = 0;
    let mut lower_border = height - 1;

    loop {
        if x == right_border && y == upper_border + 1 {
            x -= 1;
            y += 1;
            dir_x = 0;
            dir_y = 1;
            upper_border += 2
        } else if x == left_border && y == lower_border - 1 {
            x += 1;
            y -= 1;
            dir_x = 0;
            dir_y = -1;
            lower_border -= 2;
        } else if y == lower_border && x == right_border - 1 {
            y -= 1;
            x -= 1;
            dir_x = -1;
            dir_y = 0;
            right_border -= 2;
        } else if y == upper_border && x == left_border + 1 {
            y += 1;
            x += 1;
            dir_x = 1;
            dir_y = 0;
            left_border += 2;
        }
        fill[y as usize][x as usize] = '0';
        if y == last_y && x == last_x {
            break;
        }
        x += dir_x;
        y += dir_y;
        i += 1;
    }

    return fill
        .into_iter()
        .map(|line| {
            let mut spaced: Vec<char> = line
                .clone()
                .into_iter()
                .zip(" ".repeat(size as usize).chars().into_iter())
                .flat_map(|(a, b)| vec![a, b])
                .collect();
            spaced.pop();
            spaced.push('\n');
            spaced
        })
        .flatten()
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>();
}

#[cfg(test)]
mod tests {
    use crate::spiral;

    #[test]
    fn test3() {
        assert_eq!(spiral(3), "");
    }

    #[test]
    fn test5() {
        let expected = "\
            1 1 1 1 1\n\
            0 0 0 0 1\n\
            1 1 1 0 1\n\
            1 0 0 0 1\n\
            1 1 1 1 1\n";

        assert_eq!(spiral(5), expected);
    }

    #[test]
    fn test6() {
        let expected = "\
            1 1 1 1 1 1\n\
            0 0 0 0 0 1\n\
            1 1 1 1 0 1\n\
            1 0 0 1 0 1\n\
            1 0 0 0 0 1\n\
            1 1 1 1 1 1\n";

        assert_eq!(spiral(6), expected);
    }

    #[test]
    fn test7() {
        let expected = "\
            1 1 1 1 1 1 1\n\
            0 0 0 0 0 0 1\n\
            1 1 1 1 1 0 1\n\
            1 0 0 0 1 0 1\n\
            1 0 1 1 1 0 1\n\
            1 0 0 0 0 0 1\n\
            1 1 1 1 1 1 1\n";

        assert_eq!(spiral(7), expected);
    }

    #[test]
    fn test8() {
        let expected = "\
            1 1 1 1 1 1 1 1\n\
            0 0 0 0 0 0 0 1\n\
            1 1 1 1 1 1 0 1\n\
            1 0 0 0 0 1 0 1\n\
            1 0 1 0 0 1 0 1\n\
            1 0 1 1 1 1 0 1\n\
            1 0 0 0 0 0 0 1\n\
            1 1 1 1 1 1 1 1\n";

        assert_eq!(spiral(8), expected);
    }

    #[test]
    fn test9() {
        let expected = "\
            1 1 1 1 1 1 1 1 1\n\
            0 0 0 0 0 0 0 0 1\n\
            1 1 1 1 1 1 1 0 1\n\
            1 0 0 0 0 0 1 0 1\n\
            1 0 1 1 1 0 1 0 1\n\
            1 0 1 0 0 0 1 0 1\n\
            1 0 1 1 1 1 1 0 1\n\
            1 0 0 0 0 0 0 0 1\n\
            1 1 1 1 1 1 1 1 1\n";

        assert_eq!(spiral(9), expected);
    }
}
