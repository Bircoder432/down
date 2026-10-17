use tokio::fs;

pub async fn parse_file(path: &str, links: &mut Vec<String>, filenames: &mut Vec<String>) {
    let content = fs::read_to_string(path).await.unwrap();
    let lines: Vec<&str> = content.split('\n').collect();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        if parts.len() == 1 {
            links.push(parts[0].to_string());
        }

        if parts.len() >= 2 {
            links.push(parts[0].to_string());
            filenames.push(parts[1].to_string());
        }
    }
}
