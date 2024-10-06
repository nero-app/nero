fn main() {
    typewind::build("../target/classes.txt", &["./src/**/*.rs"]).expect("Failed to build classes");
}
