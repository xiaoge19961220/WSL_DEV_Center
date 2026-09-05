use crate::models::Docker;

pub fn parse_docker(output: &str) -> Result<Vec<Docker>, String> {
    output.lines().filter(|l| !l.trim().is_empty()).enumerate().map(|(i, line)| {
        serde_json::from_str(line).map_err(|e| format!("无法解析 Docker 容器数据。\n位置：容器列表第 {} 行\n详情：{e}", i + 1))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn docker_fields_are_camel_case_for_frontend() {
        let rows = parse_docker(r#"{"ID":"abc123","Image":"redis","Status":"Up 2 hours","Ports":"6379/tcp","Names":"cache","Command":"redis-server","CreatedAt":"today"}"#).unwrap();
        let value = serde_json::to_value(&rows[0]).unwrap();
        assert_eq!(value["id"], "abc123");
        assert_eq!(value["names"], "cache");
        assert_eq!(value["created"], "today");
        assert!(value.get("ID").is_none());
    }
    #[test]
    fn malformed_rows_are_not_silently_dropped() {
        assert!(parse_docker("not json").is_err());
        assert!(parse_docker("").unwrap().is_empty());
    }
}
