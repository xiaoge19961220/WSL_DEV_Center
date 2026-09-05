use crate::models::Port;

pub fn parse_ports(output: &str, netstat: bool) -> Vec<Port> {
    output.lines().filter_map(|raw| {
        let fields: Vec<_> = raw.split_whitespace().collect();
        let protocol = if fields.first()?.starts_with("tcp") { "tcp" }
            else if fields.first()?.starts_with("udp") { "udp" } else { return None; };
        if protocol == "tcp" && !fields.contains(&"LISTEN") { return None; }
        let local = *fields.get(if netstat { 3 } else { 4 })?;
        let (address, number) = local.rsplit_once(':')?;
        let port: u16 = number.parse().ok()?;
        let (pid, process_name) = if netstat {
            match fields.last()?.split_once('/') {
                Some((pid, name)) => (pid.parse().ok(), Some(name.to_owned())),
                None => (None, None),
            }
        } else {
            let pid = raw.split("pid=").nth(1).and_then(|part| part.split(|c: char| !c.is_ascii_digit()).next()).and_then(|p| p.parse().ok());
            let name = raw.split("users:((\"").nth(1).and_then(|part| part.split('"').next()).map(str::to_owned);
            (pid, name)
        };
        Some(Port { protocol: protocol.into(), local_address: address.trim_matches(['[', ']']).into(), port, process_name, pid, raw: raw.into() })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ss_ipv6_and_udp() {
        let rows = parse_ports("Netid State Recv-Q Send-Q Local Address:Port Peer Address:Port Process\ntcp LISTEN 0 128 [::]:3000 [::]:* users:((\"node\",pid=42,fd=3))\nudp UNCONN 0 0 127.0.0.1:53 0.0.0.0:*", false);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].local_address, "::");
        assert_eq!(rows[0].pid, Some(42));
        assert_eq!(rows[0].process_name.as_deref(), Some("node"));
        assert_eq!(rows[1].port, 53);
    }
    #[test]
    fn netstat_udp_without_state_and_tcp_process() {
        let rows = parse_ports("tcp 0 0 0.0.0.0:8080 0.0.0.0:* LISTEN 321/python\nudp6 0 0 :::5353 :::* 55/avahi\ninvalid", true);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].pid, Some(321));
        assert_eq!(rows[1].protocol, "udp");
        assert_eq!(rows[1].process_name.as_deref(), Some("avahi"));
    }
}
