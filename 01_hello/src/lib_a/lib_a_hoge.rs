pub fn lib_a_hoge_hoge() {
    println!("public lib_a_hoge_hoge");
    lib_a_hoge_fuga();
}

fn lib_a_hoge_fuga() {
    println!("private lib_a_hoge_fuga");
}