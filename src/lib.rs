extern crate encoding;

pub mod win {
    use std::process::Command;
    use encoding::{Encoding, DecoderTrap};
    use encoding::all::GBK;
    use super::NetworkInfo;
    pub fn get()->Vec<NetworkInfo>{
        let output = Command::new("ipconfig")
            .args(&["/all"])
            .output()
            .expect("failed to execute `ifconfig`");

        let network_info=GBK.decode(output.stdout.as_slice(), DecoderTrap::Strict).unwrap();
        pick_network_info_from_string(network_info)   
    }

    fn pick_network_info_from_string(network_info:String)->Vec<NetworkInfo>{

        let mut network_infos:Vec<NetworkInfo> = Vec::new();;
        let mut info=NetworkInfo{ip:"".to_string(),mac:"".to_string(),mask:"".to_string()};
        for line in network_info.lines() {
            if line.contains("物理地址") {
                let mac_info:Vec<&str> =line.split(":").collect();
                let mac=mac_info.get(1).unwrap_or(&"").to_string().trim().to_string(); 
                info.mac=mac;            
            }else if line.contains("IPv4 地址") {
                let ip_info:Vec<&str> =line.split(":").collect();
                let ip=ip_info.get(1).unwrap_or(&"").to_string().trim().to_string();
                info.ip=ip.trim_matches(|c| !(char::is_numeric(c) || c == '.')).to_string();
            }else if line.contains("子网掩码") {
                let mask_info:Vec<&str> =line.split(":").collect();
                let mask=mask_info.get(1).unwrap_or(&"").to_string().trim().to_string();
                info.mask=mask;            
                network_infos.push(info);
                info=NetworkInfo{ip:"".to_string(),mac:"".to_string(),mask:"".to_string()};
            }

        }

    network_infos.into_iter().filter(|net|!net.ip.is_empty()).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::NetworkInfo;

        #[test]
        fn tet_pick_network_info_from_string() {
        let network_info = r#"
    Windows IP 配置

    主机名  . . . . . . . . . . . . . : kaka
    主 DNS 后缀 . . . . . . . . . . . :
    节点类型  . . . . . . . . . . . . : 混合
    IP 路由已启用 . . . . . . . . . . : 否
    WINS 代理已启用 . . . . . . . . . : 否

    以太网适配器 以太网:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :
    描述. . . . . . . . . . . . . . . : Realtek PCIe GBE Family Controller
    物理地址. . . . . . . . . . . . . : 6C-F0-49-0D-20-1C
    DHCP 已启用 . . . . . . . . . . . : 是
    自动配置已启用. . . . . . . . . . : 是

    无线局域网适配器 本地连接* 2:

    媒体状态  . . . . . . . . . . . . : 媒体已断开连接
    连接特定的 DNS 后缀 . . . . . . . :
    描述. . . . . . . . . . . . . . . : Microsoft Wi-Fi Direct Virtual Adapter
    物理地址. . . . . . . . . . . . . : 30-B4-9E-6A-11-03
    DHCP 已启用 . . . . . . . . . . . : 是
    自动配置已启用. . . . . . . . . . : 是

    无线局域网适配器 WLAN:

    连接特定的 DNS 后缀 . . . . . . . :
    描述. . . . . . . . . . . . . . . : TP-LINK Wireless USB Adapter
    物理地址. . . . . . . . . . . . . : 30-B4-9E-6A-11-01
    DHCP 已启用 . . . . . . . . . . . : 是
    自动配置已启用. . . . . . . . . . : 是
    本地链接 IPv6 地址. . . . . . . . : fe80::189:b7d9:4ac2:e462%12(首选)
    IPv4 地址 . . . . . . . . . . . . : 192.168.31.226(首选)
    子网掩码  . . . . . . . . . . . . : 255.255.255.0
    获得租约的时间  . . . . . . . . . : 2017年7月16日 星期日 8:48:54
    租约过期的时间  . . . . . . . . . : 2017年7月17日 星期一 9:14:45
    默认网关. . . . . . . . . . . . . : 192.168.31.1
    DHCP 服务器 . . . . . . . . . . . : 192.168.31.1
    DHCPv6 IAID . . . . . . . . . . . : 120632478
    DHCPv6 客户端 DUID  . . . . . . . : 00-01-00-01-20-D5-2D-2E-6C-F0-49-0D-20-1C
    DNS 服务器  . . . . . . . . . . . : 192.168.31.1
    TCPIP 上的 NetBIOS  . . . . . . . : 已启用

    隧道适配器 Teredo Tunneling Pseudo-Interface:

    连接特定的 DNS 后缀 . . . . . . . :
    描述. . . . . . . . . . . . . . . : Teredo Tunneling Pseudo-Interface
    物理地址. . . . . . . . . . . . . : 00-00-00-00-00-00-00-E0
    DHCP 已启用 . . . . . . . . . . . : 否
    自动配置已启用. . . . . . . . . . : 是
    IPv6 地址 . . . . . . . . . . . . : 2001:0:9d38:953c:2cd7:ebb6:8d22:424b(首选)
    本地链接 IPv6 地址. . . . . . . . : fe80::2cd7:ebb6:8d22:424b%13(首选)
    默认网关. . . . . . . . . . . . . : ::
    DHCPv6 IAID . . . . . . . . . . . : 318767104
    DHCPv6 客户端 DUID  . . . . . . . : 00-01-00-01-20-D5-2D-2E-6C-F0-49-0D-20-1C
    TCPIP 上的 NetBIOS  . . . . . . . : 已禁用"#;

        assert_eq!(pick_network_info_from_string(network_info.to_string()),
        vec![NetworkInfo{ip:"192.168.31.226".to_string(),mac:"30-B4-9E-6A-11-01".to_string(),mask:"255.255.255.0".to_string()}]); 
        }
    }
}


#[derive(Debug)]
pub struct NetworkInfo {
    ip  :String,
    mac  :String,
    mask :String,
}

impl PartialEq for NetworkInfo {
    fn eq(&self, other: &NetworkInfo) -> bool {
        self.ip == other.ip
        &&self.mac == other.mac
        &&self.mask == other.mask
    }
}