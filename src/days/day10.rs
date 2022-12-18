use super::day::Day;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Noop,
    Addx(i32),
}
impl Command {
    fn run(self, x: &mut i32) {
        match self {
            Self::Noop => {}
            Self::Addx(constant) => *x += constant,
        }
    }
    const fn time(self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

pub struct Day10;
impl Day for Day10 {
    type Parsed = Vec<Command>;
    type Output = String;

    fn parse(input: String) -> Result<Self::Parsed> {
        Ok(input
            .lines()
            .map(|line| {
                let mut split = line.split(' ');
                match split.next().unwrap() {
                    "noop" => Command::Noop,
                    "addx" => Command::Addx(split.next().unwrap().parse().unwrap()),
                    _ => panic!(),
                }
            })
            .collect())
    }
    fn first(code: Self::Parsed) -> Self::Output {
        let mut x = 1;
        let mut cycles = 0;
        let mut next_check = 20;
        let mut sum_report = 0;
        for line in code {
            let future_time = cycles + line.time();
            if next_check < future_time + 1 {
                sum_report += next_check * x;
                next_check += 40;
            }
            line.run(&mut x);
            if next_check <= future_time {
                sum_report += next_check * x;
                next_check += 40;
            }
            cycles = future_time;
        }
        sum_report.to_string()
    }
    fn second(code: Self::Parsed) -> Self::Output {
        let mut x = 1;
        let mut report = String::new();
        let mut cycles = 0;
        for line in code {
            for cycles_plus in 0..line.time() {
                let x_pos = (cycles + cycles_plus) % 40;
                if x_pos == 0 && cycles != 0 {
                    report.push('\n');
                }
                if x_pos >= x-1 && x_pos <= x+1 {
                    report.push('#');
                } else {
                    report.push('.');
                }
            }
            cycles += line.time();
            line.run(&mut x);
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    fn parsed() -> <Day10 as Day>::Parsed {
        Day10::parse(INPUT.to_string()).unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(Day10::first(parsed()), "13140");
    }
    #[test]
    fn part2() {
        assert_eq!(
            Day10::second(parsed()),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
