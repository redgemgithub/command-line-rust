// main.rsはlib.rsと直接の関係はないので、外部クレートとして利用を宣言する。
// Cargo.tomlで宣言したname(クレート名)でmain.rsのバイナリができる(名前が置き換わる)のと同様に、
// Cargo.tomlで宣言したname(クレート名)で参照できるlib.rsのライブラリができる(名前が置き換わる)。
// そのため、参照する名前空間にlib自体は現れない。
use hello::lib_a::{lib_a_hoge, lib_a_fuga};

fn main() {
    println!("Hello, world!");

    // let mut cmd = std::process::Command::new("ls");
    // println!("{}", String::from_utf8_lossy(&cmd.output().unwrap().stdout));

    lib_a_hoge::lib_a_hoge_hoge();
    lib_a_fuga::lib_a_fuga_hoge();
}
