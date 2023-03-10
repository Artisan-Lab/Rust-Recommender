use version_compare::Version;

fn main() {
  let tags: Vec<String> = vec!["1.2.1".to_string(),"1.2.2".to_string()];
  let mut max_ver = Version::from("0.0.0").unwrap();

  for tag in tags.iter() { //for tag in &tags{
    let v_tag = Version::from(&tag.as_str()).unwrap();
    if v_tag > max_ver {
       max_ver = v_tag;
       }
    }
  println!("max_ver: {max_ver}");
}