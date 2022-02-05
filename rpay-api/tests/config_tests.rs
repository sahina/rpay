use op_api::conf::get_configuration;

#[test]
fn config_test() {
    let config = get_configuration().unwrap();

    println!("{:?}", config)
}