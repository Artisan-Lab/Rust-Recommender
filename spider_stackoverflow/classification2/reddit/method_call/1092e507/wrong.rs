use yaml_rust::Yaml;

fn do_it(value: &Yaml) {
    let _comment = value["comment"].into_string();
}
