use crate::models::Distro;

pub fn parse_wsl_list(output: &str) -> Vec<Distro> {
    output
        .lines()
        .filter_map(|raw| {
            let clean = raw.replace('\0', "");
            let line = clean.trim().trim_start_matches('\u{feff}');
            let is_default = line.starts_with('*');
            let parts: Vec<_> = line.trim_start_matches('*').split_whitespace().collect();
            if parts.len() < 3 {
                return None;
            }
            let version: u8 = parts.last()?.parse().ok()?;
            if !matches!(version, 1 | 2) {
                return None;
            }
            let state = match parts[parts.len() - 2] {
                "Running" | "运行中" | "正在运行" => "Running",
                "Stopped" | "已停止" | "已停止运行" => "Stopped",
                "Installing" | "安装中" | "正在安装" => "Installing",
                _ => "Unknown",
            };
            Some(Distro {
                name: parts[..parts.len() - 2].join(" "),
                state: state.into(),
                version: Some(version),
                is_default,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn supports_localized_headers_spaces_and_invalid_lines() {
        let rows = parse_wsl_list("  名称 状态 版本\n* Ubuntu 中文 运行中 2\n  Debian 已停止 1\n invalid line\n bad state 9\n  Fedora Something 2");
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].name, "Ubuntu 中文");
        assert!(rows[0].is_default);
        assert_eq!(rows[0].state, "Running");
        assert_eq!(rows[1].state, "Stopped");
        assert_eq!(rows[1].version, Some(1));
        assert_eq!(rows[2].state, "Unknown");
    }
}
