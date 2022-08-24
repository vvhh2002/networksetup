use std::io::Result;
use std::process::{Command, ExitStatus, Stdio};

const ON: &str = "on";
const OFF: &str = "off";

/// Off / On / Set new value
#[derive(Debug, Clone)]
pub enum Config<T> {
    Off,
    On,
    Value(T),
}

/// Proxy address configuration
#[derive(Debug, Clone)]
pub struct Address<'a> {
    host: &'a str,
    port: &'a str,
    auth: Option<(&'a str, &'a str)>,
}

impl<'a> Address<'a> {
    pub fn new(host: &'a str, port: &'a str) -> Self {
        Self {
            host,
            port,
            auth: None,
        }
    }

    pub fn auth(&mut self, username: &'a str, password: &'a str) -> &mut Self {
        self.auth = Some((username, password));
        self
    }
}

/// Network service
#[derive(Debug, Clone)]
pub enum Network<'a> {
    Ethernet,
    WiFi,
    BluetoothPAN,
    ThunderboltBridge,
    Name(&'a str),
}

impl<'a> Network<'a> {
    fn as_str(&self) -> &'a str {
        match self {
            Network::Ethernet => "Ethernet",
            Network::WiFi => "Wi-Fi",
            Network::BluetoothPAN => "Bluetooth PAN",
            Network::ThunderboltBridge => "Thunderbolt Bridge",
            Network::Name(s) => s,
        }
    }
}

fn cmd() -> Command {
    let mut cmd = Command::new("networksetup");
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd
}

/// macOS Proxies: Atuo Proxy Discovery
pub fn auto_proxy_discovery(network: Network, enable: bool) -> Result<ExitStatus> {
    let mut cmd = cmd();
    cmd.args(&["-setproxyautodiscovery", network.as_str()]);
    if enable {
        cmd.arg(ON);
    } else {
        cmd.arg(OFF);
    }
    cmd.status()
}

/// macOS Proxies: Atuomatic Proxy Configuration
pub fn auto_proxy(network: Network, url: Config<&str>) -> Result<ExitStatus> {
    let mut cmd = cmd();
    match url {
        Config::Off => {
            cmd.args(&["-setautoproxystate", network.as_str(), OFF]);
        }
        Config::On => {
            cmd.args(&["-setautoproxystate", network.as_str(), ON]);
        }
        Config::Value(url) => {
            cmd.args(&["-setautoproxyurl", network.as_str(), url]);
        }
    }
    cmd.status()
}

/// macOS Proxies: FTP Proxy
pub fn ftp_proxy(network: Network, setup: Config<&Address>) -> Result<ExitStatus> {
    let mut cmd = cmd();
    match setup {
        Config::Off => {
            cmd.args(&["-setftpproxystate", network.as_str(), OFF]);
        }
        Config::On => {
            cmd.args(&["-setftpproxystate", network.as_str(), ON]);
        }
        Config::Value(addr) => {
            let mut ops = vec!["-setftpproxy", network.as_str(), addr.host, addr.port];
            if let Some((username, password)) = addr.auth {
                ops.extend_from_slice(&[ON, username, password]);
            }
            cmd.args(&ops);
        }
    }
    cmd.status()
}

/// macOS Proxies: Web Proxy (HTTP)
pub fn web_proxy(network: Network, setup: Config<&Address>) -> Result<ExitStatus> {
    let mut cmd = cmd();
    match setup {
        Config::Off => {
            cmd.args(&["-setwebproxystate", network.as_str(), OFF]);
        }
        Config::On => {
            cmd.args(&["-setwebproxystate", network.as_str(), ON]);
        }
        Config::Value(addr) => {
            let mut ops = vec!["-setwebproxy", network.as_str(), addr.host, addr.port];
            if let Some((username, password)) = addr.auth {
                ops.extend_from_slice(&["on", username, password]);
            }
            cmd.args(&ops);
        }
    }
    cmd.status()
}

/// macOS Proxies: Secure Web Proxy (HTTPS)
pub fn secure_web_proxy(network: Network, setup: Config<&Address>) -> Result<ExitStatus> {
    let mut cmd = cmd();
    match setup {
        Config::Off => {
            cmd.args(&["-setsecurewebproxystate", network.as_str(), OFF]);
        }
        Config::On => {
            cmd.args(&["-setsecurewebproxystate", network.as_str(), ON]);
        }
        Config::Value(addr) => {
            let mut ops = vec!["-setsecurewebproxy", network.as_str(), addr.host, addr.port];
            if let Some((username, password)) = addr.auth {
                ops.extend_from_slice(&[ON, username, password]);
            }
            cmd.args(&ops);
        }
    }
    cmd.status()
}

/// macOS Proxies: Socks Proxy
pub fn socks_proxy(network: Network, setup: Config<&Address>) -> Result<ExitStatus> {
    let mut cmd = cmd();
    match setup {
        Config::Off => {
            cmd.args(&["-setsocksfirewallproxystate", network.as_str(), "\"\"","\"\""]);
            cmd.args(&["-setsocksfirewallproxystate", network.as_str(), OFF]);
        }
        Config::On => {
            cmd.args(&["-setsocksfirewallproxystate", network.as_str(), ON]);
        }
        Config::Value(addr) => {
            let mut ops = vec![
                "-setsocksfirewallproxy",
                network.as_str(),
                addr.host,
                addr.port,
            ];
            if let Some((username, password)) = addr.auth {
                ops.extend_from_slice(&[ON, username, password]);
            }
            cmd.args(&ops);
        }
    }
    cmd.status()
}

/// macOS Proxies: Bypass proxy settings for these Hosts & Domains
pub fn proxy_by_pass_domain(network: Network, hosts: &[&str]) -> Result<ExitStatus> {
    let mut cmd = cmd();
    cmd.args(&["-setproxybypassdomains", network.as_str()]);
    if hosts.is_empty() {
        cmd.arg("Empty");
    } else {
        cmd.args(hosts);
    }
    cmd.status()
}

/// macOS DNS
pub fn dns_server(network: Network, hosts: &[&str]) -> Result<ExitStatus> {
    let mut cmd = cmd();
    cmd.args(&["-setdnsservers", network.as_str()]);
    if hosts.is_empty() {
        cmd.arg("Empty");
    } else {
        cmd.args(hosts);
    }
    cmd.status()
}
