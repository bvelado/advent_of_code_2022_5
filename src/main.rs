use std::fs;

pub type CrateStackState = Vec<char>;

trait RemoveCrates {
    fn remove_crates(&mut self, count: usize) -> CrateStackState;
}

trait PutCrates {
    fn put_crates(&mut self, crates: &mut CrateStackState);
}

trait RemoveCratesInOrder {
    fn remove_crates_in_order(&mut self, count: usize) -> CrateStackState;
}

impl RemoveCrates for CrateStackState {
    fn remove_crates(&mut self, count: usize) -> CrateStackState {
        let mut temp: CrateStackState = vec![];
        for _ in 0..count {
            temp.push(self.pop().unwrap());
        }
        temp
    }
}

impl RemoveCratesInOrder for CrateStackState {
    fn remove_crates_in_order(&mut self, count: usize) -> CrateStackState {
        self.drain(self.len() - count..).collect()
    }
}

impl PutCrates for CrateStackState {
    fn put_crates(&mut self, crates: &mut CrateStackState) {
        self.append(crates);
    }
}

pub struct RearrangementProcedure {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

fn main() {
    let input = read_input();
    let mut initial_state: Vec<CrateStackState> = vec![
        vec!['R', 'C', 'H'],
        vec!['F', 'S', 'L', 'H', 'J', 'B'],
        vec!['Q', 'T', 'J', 'H', 'D', 'M', 'R'],
        vec!['J', 'B', 'Z', 'H', 'R', 'G', 'S'],
        vec!['B', 'C', 'D', 'T', 'Z', 'F', 'P', 'R'],
        vec!['G', 'C', 'H', 'T'],
        vec!['L', 'W', 'P', 'B', 'Z', 'V', 'N', 'S'],
        vec!['C', 'G', 'Q', 'J', 'R'],
        vec!['S', 'F', 'P', 'H', 'R', 'T', 'D', 'L'],
    ];
    reverse_array_in_place_to_fix_my_mistake(&mut initial_state);
    println!("Part 1 : {}", part_one(&input, &initial_state, 10));
    println!("Part 2 : {}", part_two(&input, &initial_state, 10));
}

pub fn part_one(
    input: &String,
    initial_state: &Vec<CrateStackState>,
    skip_lines_count: usize,
) -> String {
    let mut stacks = initial_state.clone();
    let procedures_input: Vec<&str> = input.lines().skip(skip_lines_count).collect();

    let procedures = procedures_input
        .iter()
        .map(|&s| parse_rearrangement_procedure(s));

    for p in procedures {
        let mut moved_crates = stacks[p.from].remove_crates(p.count);
        stacks[p.to].put_crates(&mut moved_crates);
    }

    String::from(format_output(&stacks))
}

pub fn part_two(
    input: &String,
    initial_state: &Vec<CrateStackState>,
    skip_lines_count: usize,
) -> String {
    let mut stacks = initial_state.clone();
    let procedures_input: Vec<&str> = input.lines().skip(skip_lines_count).collect();

    let procedures = procedures_input
        .iter()
        .map(|&s| parse_rearrangement_procedure(s));

    for p in procedures {
        let mut moved_crates = stacks[p.from].remove_crates_in_order(p.count);
        stacks[p.to].put_crates(&mut moved_crates);
    }

    String::from(format_output(&stacks))
}

pub fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Error reading input file")
}

pub fn parse_rearrangement_procedure(input: &str) -> RearrangementProcedure {
    let values: Vec<usize> = input
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .split(' ')
        .map(|c| c.parse().unwrap())
        .collect();
    RearrangementProcedure {
        count: values[0],
        from: values[1] - 1,
        to: values[2] - 1,
    }
}

pub fn format_output(input: &Vec<CrateStackState>) -> String {
    input
        .iter()
        .map(|s| s.last().unwrap())
        .fold(String::from(""), |mut acc, cur| {
            acc.push(cur.clone());
            acc
        })
}

/// It just reverses the order of the crates in every stack
/// Because I typed then in the wrong order and I do not want
/// to type it again.
///
/// # Examples
///
/// ```
/// use aoc_2022_5::reverse_array_in_place_to_fix_my_mistake;
///
/// let mut input = vec![1,2,3];
/// reverse_array_in_place_to_fix_my_mistake(&mut input);
/// assert_eq!(input, [3,2,1]);
/// ```
pub fn reverse_array_in_place_to_fix_my_mistake(input: &mut Vec<CrateStackState>) {
    for i in input {
        i.reverse()
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    use crate::{part_one, part_two, reverse_array_in_place_to_fix_my_mistake, CrateStackState};

    fn read_test_input() -> String {
        fs::read_to_string("input_test.txt").expect("Error reading test input file")
    }

    fn build_test_initial_state() -> Vec<CrateStackState> {
        vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']]
    }

    #[test]
    pub fn should_return_CMZ_given_test_input() {
        let input = read_test_input();
        let mut init_state = build_test_initial_state();
        reverse_array_in_place_to_fix_my_mistake(&mut init_state);
        assert_eq!("CMZ", part_one(&input, &init_state, 5));
    }

    #[test]
    pub fn should_return_MCD_given_test_input() {
        let input = read_test_input();
        let mut init_state = build_test_initial_state();
        reverse_array_in_place_to_fix_my_mistake(&mut init_state);
        assert_eq!("MCD", part_two(&input, &init_state, 5));
    }
}
