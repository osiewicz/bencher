use std::{collections::BTreeMap, str::FromStr, time::Duration};

use bencher_json::project::report::{
    metric_kind::LATENCY_SLUG,
    new::{JsonBenchmarksMap, JsonMetrics},
    JsonMetric,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{digit1, line_ending, space1},
    combinator::{map, success},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};

use crate::{Adapter, AdapterError};

pub struct AdapterRustBench;

impl Adapter for AdapterRustBench {
    fn convert(input: &str) -> Result<JsonBenchmarksMap, AdapterError> {
        parse_rust_bench(input)
            .map(|(_, benchmarks)| benchmarks)
            .map_err(|err| AdapterError::Nom(err.map_input(Into::into)))
    }
}

enum Test {
    Ignored,
    Bench(JsonMetric),
}

fn parse_rust_bench(input: &str) -> IResult<&str, JsonBenchmarksMap> {
    map(
        tuple((
            line_ending,
            // running X test(s)
            tag("running"),
            space1,
            digit1,
            space1,
            alt((tag("tests"), tag("test"))),
            line_ending,
            // test rust::mod::path::to_test ... ignored/Y ns/iter (+/- Z)
            many0(tuple((
                tag("test"),
                space1,
                take_until1(" "),
                space1,
                tag("..."),
                space1,
                alt((
                    map(tag("ignored"), |_| Test::Ignored),
                    map(parse_bench, Test::Bench),
                )),
                line_ending,
            ))),
            line_ending,
        )),
        |(_, _, _, _, _, _, _, benches, _)| {
            let mut benchmarks = BTreeMap::new();
            for bench in benches {
                if let Some((benchmark, latency)) = to_latency(bench) {
                    let mut inner = BTreeMap::new();
                    inner.insert(LATENCY_SLUG.into(), latency);
                    benchmarks.insert(benchmark, JsonMetrics { inner });
                }
            }
            benchmarks.into()
        },
    )(input)
}

fn to_latency(
    bench: (&str, &str, &str, &str, &str, &str, Test, &str),
) -> Option<(String, JsonMetric)> {
    let (_, _, key, _, _, _, test, _) = bench;
    match test {
        Test::Ignored => None,
        Test::Bench(metric) => Some((key.into(), metric)),
    }
}

pub enum Units {
    Nano,
    Micro,
    Milli,
    Sec,
}

impl From<&str> for Units {
    fn from(time: &str) -> Self {
        match time {
            "ns" => Self::Nano,
            "μs" => Self::Micro,
            "ms" => Self::Milli,
            "s" => Self::Sec,
            _ => panic!("Unexpected time abbreviation"),
        }
    }
}

fn parse_bench(input: &str) -> IResult<&str, JsonMetric> {
    map(
        tuple((
            tag("bench:"),
            space1,
            parse_number,
            space1,
            take_until1("/"),
            tag("/iter"),
            space1,
            tag("(+/-"),
            space1,
            parse_number,
            tag(")"),
        )),
        |(_, _, duration, _, units, _, _, _, _, variance, _)| {
            let units = Units::from(units);
            let value = (to_duration(to_u64(duration), &units).as_nanos() as f64).into();
            let variance = Some((to_duration(to_u64(variance), &units).as_nanos() as f64).into());
            JsonMetric {
                value,
                lower_bound: variance,
                upper_bound: variance,
            }
        },
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(tuple((digit1, alt((tag(","), success(" "))))))(input)
}

fn to_u64(input: Vec<(&str, &str)>) -> u64 {
    let mut number = String::new();
    for (digit, _) in input {
        number.push_str(digit);
    }
    u64::from_str(&number).unwrap()
}

fn to_duration(time: u64, units: &Units) -> Duration {
    match units {
        Units::Nano => Duration::from_nanos(time),
        Units::Micro => Duration::from_micros(time),
        Units::Milli => Duration::from_millis(time),
        Units::Sec => Duration::from_secs(time),
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::AdapterRustBench;
    use crate::Adapter;

    macro_rules! convert_file {
        ($adapter:ident, $dir_path:expr, $file_name:expr, $count:expr) => {
            paste::paste! {
                #[test]
                fn [<test_ $adapter:snake _ $count>](){
                    convert_file_name::<$adapter>($dir_path, $file_name);
                }
            }
        };
    }

    pub(crate) use convert_file;

    const RUST_DIR_PATH: &str = "./tool_output/rust";

    fn convert_rust_bench(file_name: &str) {
        convert_file_name::<AdapterRustBench>(RUST_DIR_PATH, file_name);
    }

    fn convert_file_name<A>(dir_path: &str, file_name: &str)
    where
        A: Adapter,
    {
        let dir_path = Path::new(dir_path);
        let file_path = dir_path.join(file_name);
        convert_file_path::<A>(file_path.to_string_lossy().as_ref());
    }

    fn convert_file_path<A>(file_path: &str)
    where
        A: Adapter,
    {
        let contents = std::fs::read_to_string(file_path)
            .expect(&format!("Failed to read test file: {file_path}"));
        A::convert(&contents).expect(&format!("Failed to convert contents: {contents}"));
    }

    #[test]
    fn test_adapter_rust_zero() {
        convert_rust_bench("cargo_bench_0.txt");
    }

    #[test]
    fn test_adapter_rust_one() {
        convert_rust_bench("cargo_bench_1.txt");
    }

    #[test]
    fn test_adapter_rust_two() {
        convert_rust_bench("cargo_bench_2.txt");
    }

    convert_file!(AdapterRustBench, RUST_DIR_PATH, "cargo_bench_0.txt", 0);
}
