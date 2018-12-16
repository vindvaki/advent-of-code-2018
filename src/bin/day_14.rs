use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let recipe_count: usize = data.parse().unwrap();
    println!("part_1: {}", part_1(recipe_count));
    println!("part_2: {}", part_2(recipe_count.to_string().as_str()));
}

#[derive(Debug, Clone)]
pub struct State {
    elves: (usize, usize),
    recipes: Vec<usize>,
}

impl State {
    pub fn new() -> State {
        State {
            elves: (0, 1),
            recipes: vec![3, 7],
        }
    }

    pub fn next(&mut self) {
        let mut sum = self.recipes[self.elves.0] + self.recipes[self.elves.1];
        let mut digits = Vec::new();
        loop {
            digits.push(sum % 10);
            sum /= 10;
            if sum == 0 {
                break;
            }
        }
        digits.reverse();
        for &d in digits.iter() {
            self.recipes.push(d);
        }

        let n = self.recipes.len();
        let a = self.elves.0;
        let b = self.elves.1;
        self.elves = ((a + self.recipes[a] + 1) % n, (b + self.recipes[b] + 1) % n);
    }

    pub fn len(&self) -> usize {
        self.recipes.len()
    }
}

fn part_1(recipe_count: usize) -> String {
    let mut state = State::new();
    while state.len() < recipe_count + 10 {
        state.next();
    }
    state.recipes[recipe_count..recipe_count + 10]
        .iter()
        .map(usize::to_string)
        .collect()
}

fn part_2(pattern_str: &str) -> usize {
    let mut state = State::new();
    let pattern: Vec<usize> = pattern_str
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect();

    let mut begin = 0;
    let mut end;
    loop {
        // state.recipes[begin..end] == pattern[0..begin-end]
        while begin < state.recipes.len() && state.recipes[begin] != pattern[0] {
            begin += 1;
        }
        end = begin;
        while end < state.recipes.len()
            && end - begin < pattern.len()
            && state.recipes[end] == pattern[end - begin]
        {
            end += 1;
        }
        if end - begin == pattern.len() {
            return begin;
        }
        if end < state.recipes.len() {
            begin += 1;
        }

        state.next();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use part_1;
        assert_eq!("5158916779", part_1(9));
        assert_eq!("0124515891", part_1(5));
        assert_eq!("9251071085", part_1(18));
        assert_eq!("5941429882", part_1(2018));
    }

    #[test]
    fn test_part_2() {
        use part_2;
        assert_eq!(9, part_2("51589"));
        assert_eq!(5, part_2("01245"));
        assert_eq!(18, part_2("92510"));
        assert_eq!(2018, part_2("59414"));
    }
}
