use svctl_proto::dev::vsx::svctl::v1::Config;

pub fn show(config: Option<Config>) {
    print!("{:?}", config)
}
