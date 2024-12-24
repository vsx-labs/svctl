use svctl_proto::dev::vsx::svctl::v1;

fn main() {
    let config = v1::Config::default();
    println!("Hello, world!: {:#?}", config);
}
