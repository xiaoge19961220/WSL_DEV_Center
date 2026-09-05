use crate::models::{Distro, OnlineDistro};

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

pub fn parse_online_distros(output: &str) -> Vec<OnlineDistro> {
    let mut in_table = false;
    output
        .lines()
        .filter_map(|raw| {
            let line = raw.replace('\0', "");
            let line = line.trim().trim_start_matches('\u{feff}');
            let upper = line.to_ascii_uppercase();
            if (upper.contains("NAME") && upper.contains("FRIENDLY"))
                || (line.contains("名称") && line.contains("友好"))
            {
                in_table = true;
                return None;
            }
            if !in_table || line.is_empty() || line.chars().all(|c| c == '-' || c.is_whitespace()) {
                return None;
            }
            let split = line.find(char::is_whitespace)?;
            let name = line[..split].trim();
            let friendly_name = line[split..].trim();
            if name.is_empty() || friendly_name.is_empty() {
                return None;
            }
            Some(OnlineDistro {
                name: name.to_owned(),
                friendly_name: friendly_name.to_owned(),
            })
        })
        .collect()
}

pub fn parse_os_release(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        line.strip_prefix("PRETTY_NAME=")
            .map(|value| value.trim().trim_matches(['\'', '"']).to_owned())
            .filter(|value| !value.is_empty())
    })
}

pub fn parse_cpu_summary(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let trimmed = line.trim();
        if !trimmed.contains("Cpu(s)") && !trimmed.starts_with("CPU:") {
            return None;
        }
        trimmed
            .split_once(':')
            .map(|(_, value)| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    })
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

    #[test]
    fn parses_online_distros_and_ignores_headers() {
        let rows = parse_online_distros(
            "以下是可安装的有效发行版列表。\nNAME              FRIENDLY NAME\nUbuntu-24.04      Ubuntu 24.04 LTS\nDebian            Debian GNU/Linux\n",
        );
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "Ubuntu-24.04");
        assert_eq!(rows[0].friendly_name, "Ubuntu 24.04 LTS");
        assert_eq!(rows[1].name, "Debian");
    }

    #[test]
    fn extracts_system_and_cpu_summaries() {
        assert_eq!(
            parse_os_release("NAME=Ubuntu\nPRETTY_NAME=\"Ubuntu 24.04.1 LTS\"\n"),
            Some("Ubuntu 24.04.1 LTS".into())
        );
        assert_eq!(
            parse_cpu_summary(
                "top - 10:00:00 up 1 day\n%Cpu(s):  1.2 us,  0.8 sy,  0.0 ni, 98.0 id\nMiB Mem : 1000 total"
            ),
            Some("1.2 us,  0.8 sy,  0.0 ni, 98.0 id".into())
        );
    }
}
