use std::{fmt::Display, fs, str::FromStr};

#[derive(Debug)]
struct Test {
    answer: i64,
    inputs: Vec<i64>,
}

fn main() {
    let raw_tests = fs::read_to_string("../../assets/tests.txt").unwrap();
    let tests: Vec<Test> = raw_tests
        .trim()
        .split("\n")
        .map(|test| {
            let parsed_test = test.trim().split(": ").collect::<Vec<&str>>();
            let answer = parsed_test[0].parse::<i64>().unwrap();
            let inputs = parsed_test[1]
                .trim()
                .split(" ")
                .map(|input| input.parse::<i64>().unwrap())
                .collect();
            Test { answer, inputs }
        })
        .collect();

    let mut test_sum = 0;

    for test in tests {
        let passes = run_test(test.answer, &test.inputs, 0);
        if passes {
            test_sum += test.answer
        }
    }

    println!("{test_sum}")
}

fn concat<A>(x: A, y: A) -> A
where
    A: Display + FromStr,
    <A as std::str::FromStr>::Err: std::fmt::Debug,
{
    (x.to_string() + &y.to_string()).parse().unwrap()
}

fn run_test(answer: i64, inputs: &[i64], so_far: i64) -> bool {
    if inputs.len() == 1 {
        let add_correct = inputs[0] + so_far == answer;
        let mult_correct = inputs[0] * so_far == answer;
        let concat_correct = concat(so_far, inputs[0]) == answer;
        if add_correct || mult_correct || concat_correct {
            return true;
        }
        return false;
    }

    let add_res = run_test(answer, &inputs[1..], so_far + inputs[0]);
    let mult_res = run_test(answer, &inputs[1..], so_far * inputs[0]);
    let concat_res = run_test(answer, &inputs[1..], concat(so_far, inputs[0]));

    add_res || mult_res || concat_res
}
