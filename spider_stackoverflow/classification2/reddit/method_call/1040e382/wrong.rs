pub fn api_request(
    agent: &ureq::Agent,
    method: &str,
    url: String,
    files: Option<Vec<File>>,
    form_data: Option<&[(&str, &str)]>,
) -> Result<ureq::Response, ureq::Error> {
    fn get_basic_auth_header(user: &str, pass: &str) -> String {
        let usr_pw = String::from(user) + ":" + pass;
        String::from("Basic ") + &base64::encode(usr_pw.as_bytes())
    }
    let request = match method {
        "DELETE" => agent.delete(url.as_str()),
        "GET" => agent.get(url.as_str()),
        "POST" => agent.post(url.as_str()),
        "PUT" => agent.put(url.as_str()),
        _ => unreachable!(),
    };

    if form_data.is_some() {
        request.send_form(form_data.unwrap());
    }
    if files.is_some() {
        for file in files.unwrap() {
            let metadata = file.metadata()?;
            let buffered_reader = BufReader::new(file);
            request
                .set("Content-Length", &metadata.len().to_string())
                .send(buffered_reader);
        }
    }
    request
        .set("Accept", "application/json")
        .set("Authorization", &get_basic_auth_header("admin", ""))
        .call()
}