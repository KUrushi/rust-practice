use clap::Clap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clap, Debug)]
#[clap(
    name = "My PRN program",
    version = "1.0.0",
    name = "KUrushi",
    about = "Super awesome sample PRN calculator"
)]

struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in PRN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

// 逆ポーランド記法計算ロジック
// 構造体雛形の作成
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    pub fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        0
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

fn main() {
    /// 1. コマンドラインから入力ファイルを指定または何も指定せず、起動する
    /// 2. 入力を読み込む準備をする。入力ファイルが指定された時は該当ファイルから、何も指定されなかった時は標準入力から読み込めるようhにする
    /// 3. 2で準備したものから１行ずつ読み込んでRPN計算処理関数に渡す
    /// 4. 計算処理関数で処理した結果を標準出力に書き出す
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        println!("No file is specified");
    }
}
