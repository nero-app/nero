fn main() {
    typewind::build("../target/components_classes.txt", &["./src/**/*.rs"])
        .expect("Failed to build classes");
}
