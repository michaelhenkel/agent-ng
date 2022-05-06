fn main() {
    tonic_build::configure()
    .build_client(true)
    .out_dir("src/protos")
    .include_file("mod.rs")
    .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
    //.client_attribute("ConfigController", "#[derive(PartialEq)]")
    .compile(
        &["../../../../src/github.com/michaelhenkel/config-controller/pkg/apis/v1/controller.proto"],
        &["../../../../src"],
    ).unwrap();
    tonic_build::configure()
    .build_client(false)
    .build_server(true)
    .out_dir("src/config_controller/cli/protos")
    .include_file("mod.rs")
    .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
    //.client_attribute("ConfigController", "#[derive(PartialEq)]")
    .compile(
        &["src/config_controller/cli/protos/cli.proto"],
        &["src/config_controller/cli/protos"],
    ).unwrap();
}