#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    #[test]
    fn test_ip_validation_logic() {
        let valid_ips = vec!["192.168.1.1", "127.0.0.1", "8.8.8.8", "::1", "2001:db8::1"];

        for ip in valid_ips {
            assert!(
                ip.parse::<IpAddr>().is_ok(),
                "Expected '{}' to be a valid IP",
                ip
            );
        }
    }

    #[test]
    fn test_ip_address_validation() {
        let valid_ipv4 = "192.168.1.1";
        assert!(valid_ipv4.parse::<IpAddr>().is_ok());

        let valid_ipv4_2 = "127.0.0.1";
        assert!(valid_ipv4_2.parse::<IpAddr>().is_ok());

        let valid_ipv6 = "::1";
        assert!(valid_ipv6.parse::<IpAddr>().is_ok());

        let valid_ipv6_2 = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
        assert!(valid_ipv6_2.parse::<IpAddr>().is_ok());
    }

    #[test]
    fn test_invalid_ip_addresses() {
        let invalid_ips = vec![
            "256.256.256.256",
            "192.168.1",
            "192.168.1.1.1",
            "abc.def.ghi.jkl",
            "",
            "192.168.1.-1",
            "not_an_ip",
            "192.168.1.256",
        ];

        for invalid_ip in invalid_ips {
            assert!(
                invalid_ip.parse::<IpAddr>().is_err(),
                "Expected '{}' to be invalid IP",
                invalid_ip
            );
        }
    }

    #[test]
    fn test_string_processing() {
        let test_inputs = vec![
            ("  hello world  ", "hello world"),
            ("\n\ttest\n\t", "test"),
            ("", ""),
            ("no_trim_needed", "no_trim_needed"),
            ("  \n  ", ""),
        ];

        for (input, expected) in test_inputs {
            let result = input.trim().to_string();
            assert_eq!(
                result, expected,
                "String processing failed for input: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_common_ip_ranges() {
        let common_ips = vec![
            "0.0.0.0",
            "127.0.0.1",
            "192.168.0.1",
            "10.0.0.1",
            "172.16.0.1",
            "255.255.255.255",
            "8.8.8.8",
            "1.1.1.1",
        ];

        for ip in common_ips {
            assert!(
                ip.parse::<IpAddr>().is_ok(),
                "Expected '{}' to be a valid IP",
                ip
            );
        }
    }

    #[test]
    fn test_ipv6_formats() {
        let ipv6_addresses = vec![
            "::1",
            "2001:db8::1",
            "2001:0db8:0000:0000:0000:0000:0000:0001",
            "::",
        ];

        for ipv6 in ipv6_addresses {
            assert!(
                ipv6.parse::<IpAddr>().is_ok(),
                "Expected '{}' to be a valid IPv6",
                ipv6
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        let edge_cases = vec![
            ("192.168.001.001", false),
            ("192.168.1.01", false),
            ("192.168.1.0", true),
            ("0.0.0.1", true),
        ];

        for (ip, should_be_valid) in edge_cases {
            let is_valid = ip.parse::<IpAddr>().is_ok();
            assert_eq!(is_valid, should_be_valid, "IP '{}' validation mismatch", ip);
        }
    }
}
