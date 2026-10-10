//! Who is the client, and did it arrive over TLS? Both questions are
//! answered from the **socket peer**, and only a peer listed in
//! `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES` (comma-separated IPs or
//! CIDR ranges, e.g. `10.0.0.0/8,::1`) is allowed to say otherwise through
//! `X-Forwarded-For` / `X-Forwarded-Proto`. An unset list trusts nobody, so a
//! spoofed forwarding header from a direct client is ignored.
//!
//! Shared by the rate limiter (`crate::rate_limit`) and the HSTS decision in
//! `crate::http_security`.

use axum::extract::ConnectInfo;
use axum::http::{Extensions, HeaderMap};
use std::net::{IpAddr, SocketAddr};

/// The socket peer of a request, when the server recorded one (the real
/// server does; an in-process test transport may not).
#[must_use]
pub fn peer_ip(extensions: &Extensions) -> Option<IpAddr> {
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}

/// One trusted address or CIDR range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cidr {
    addr: IpAddr,
    prefix: u8,
}

impl Cidr {
    fn parse(text: &str) -> Result<Self, String> {
        let (host, prefix) = match text.split_once('/') {
            Some((host, prefix)) => (host, Some(prefix)),
            None => (text, None),
        };
        let addr: IpAddr = host
            .trim()
            .parse()
            .map_err(|_| format!("not an IP address: {text:?}"))?;
        let addr = addr.to_canonical();
        let max = if addr.is_ipv4() { 32 } else { 128 };
        let prefix = match prefix {
            None => max,
            Some(p) => p
                .trim()
                .parse::<u8>()
                .ok()
                .filter(|p| *p <= max)
                .ok_or_else(|| format!("bad prefix length in {text:?}"))?,
        };
        Ok(Self { addr, prefix })
    }

    fn contains(&self, ip: IpAddr) -> bool {
        match (self.addr, ip.to_canonical()) {
            (IpAddr::V4(net), IpAddr::V4(ip)) => {
                masked(u128::from(u32::from(net)), self.prefix, 32)
                    == masked(u128::from(u32::from(ip)), self.prefix, 32)
            }
            (IpAddr::V6(net), IpAddr::V6(ip)) => {
                masked(u128::from(net), self.prefix, 128)
                    == masked(u128::from(ip), self.prefix, 128)
            }
            _ => false,
        }
    }
}

/// The top `prefix` bits of a `width`-bit address, others zeroed.
fn masked(value: u128, prefix: u8, width: u8) -> u128 {
    if prefix == 0 {
        0
    } else {
        let host_bits = u32::from(width - prefix);
        (value >> host_bits) << host_bits
    }
}

/// The set of peers allowed to assert a client address / scheme.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrustedProxies(Vec<Cidr>);

impl TrustedProxies {
    /// Trust nobody (the default).
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    /// Parse a comma-separated list of IPs / CIDR ranges.
    ///
    /// # Errors
    ///
    /// The first entry that is not a valid address or range.
    pub fn parse(list: &str) -> Result<Self, String> {
        list.split(',')
            .map(str::trim)
            .filter(|e| !e.is_empty())
            .map(Cidr::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }

    /// Read `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES`. A bad entry
    /// aborts the parse, logs a warning and trusts nobody: failing toward
    /// *less* trust is the safe direction.
    #[must_use]
    pub fn from_env() -> Self {
        let raw = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES").unwrap_or_default();
        Self::parse(&raw).unwrap_or_else(|error| {
            tracing::warn!(%error, "PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES ignored; trusting no proxy");
            Self::none()
        })
    }

    /// Whether `ip` is a trusted proxy.
    #[must_use]
    pub fn contains(&self, ip: IpAddr) -> bool {
        self.0.iter().any(|c| c.contains(ip))
    }

    /// The client address: the socket `peer`, unless the peer is a trusted
    /// proxy, in which case the right-most `X-Forwarded-For` hop that is not
    /// itself a trusted proxy (the right end is the part our own
    /// infrastructure appended; the left end is client-controlled). An
    /// unparsable hop stops the walk at the proxy. `None` only when there is
    /// no peer (an in-process test transport).
    #[must_use]
    pub fn client_ip(&self, peer: Option<IpAddr>, headers: &HeaderMap) -> Option<IpAddr> {
        let peer = peer?.to_canonical();
        if !self.contains(peer) {
            return Some(peer);
        }
        let hops: Vec<&str> = headers
            .get_all("x-forwarded-for")
            .iter()
            .filter_map(|v| v.to_str().ok())
            .flat_map(|v| v.split(','))
            .map(str::trim)
            .filter(|h| !h.is_empty())
            .collect();
        for hop in hops.iter().rev() {
            match hop.parse::<IpAddr>() {
                Ok(ip) if self.contains(ip) => {}
                Ok(ip) => return Some(ip.to_canonical()),
                Err(_) => return Some(peer),
            }
        }
        Some(peer)
    }

    /// Whether the request reached the edge over TLS: the peer is a trusted
    /// proxy and says `X-Forwarded-Proto: https`.
    #[must_use]
    pub fn is_https(&self, peer: Option<IpAddr>, headers: &HeaderMap) -> bool {
        peer.is_some_and(|p| self.contains(p))
            && headers
                .get("x-forwarded-proto")
                .and_then(|v| v.to_str().ok())
                .is_some_and(|v| v.trim().eq_ignore_ascii_case("https"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    fn xff(value: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", value.parse().unwrap());
        h
    }

    #[test]
    fn parses_addresses_and_ranges() {
        let t = TrustedProxies::parse("10.0.0.0/8, 192.168.1.5 ,::1,fd00::/8").unwrap();
        assert!(t.contains(ip("10.9.8.7")));
        assert!(t.contains(ip("192.168.1.5")));
        assert!(!t.contains(ip("192.168.1.6")));
        assert!(t.contains(ip("::1")));
        assert!(t.contains(ip("fd12::1")));
        assert!(!t.contains(ip("11.0.0.1")));
        assert!(
            t.contains(ip("::ffff:10.1.2.3")),
            "mapped v4 is canonicalized"
        );
        assert!(TrustedProxies::parse("").unwrap().0.is_empty());
        assert!(TrustedProxies::parse("nope").is_err());
        assert!(TrustedProxies::parse("10.0.0.0/33").is_err());
        assert!(
            TrustedProxies::parse("0.0.0.0/0")
                .unwrap()
                .contains(ip("8.8.8.8"))
        );
    }

    #[test]
    fn untrusted_peer_cannot_spoof_the_client_address() {
        let t = TrustedProxies::parse("10.0.0.1").unwrap();
        let got = t.client_ip(Some(ip("203.0.113.9")), &xff("1.2.3.4"));
        assert_eq!(got, Some(ip("203.0.113.9")));
        let nobody = TrustedProxies::none();
        assert_eq!(
            nobody.client_ip(Some(ip("10.0.0.1")), &xff("1.2.3.4")),
            Some(ip("10.0.0.1"))
        );
    }

    #[test]
    fn trusted_peer_forwards_the_rightmost_untrusted_hop() {
        let t = TrustedProxies::parse("10.0.0.0/8").unwrap();
        let peer = Some(ip("10.0.0.1"));
        assert_eq!(
            t.client_ip(peer, &xff("198.51.100.7")),
            Some(ip("198.51.100.7"))
        );
        // A client-supplied left hop is ignored: the right-most untrusted wins.
        assert_eq!(
            t.client_ip(peer, &xff("6.6.6.6, 198.51.100.7, 10.0.0.2")),
            Some(ip("198.51.100.7"))
        );
        assert_eq!(t.client_ip(peer, &HeaderMap::new()), peer);
        assert_eq!(t.client_ip(peer, &xff("garbage")), peer);
        assert_eq!(t.client_ip(None, &xff("1.2.3.4")), None);
    }

    #[test]
    fn https_is_believed_only_from_a_trusted_proxy() {
        let t = TrustedProxies::parse("10.0.0.1").unwrap();
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-proto", "https".parse().unwrap());
        assert!(t.is_https(Some(ip("10.0.0.1")), &h));
        assert!(!t.is_https(Some(ip("203.0.113.9")), &h));
        assert!(!t.is_https(None, &h));
        h.insert("x-forwarded-proto", "http".parse().unwrap());
        assert!(!t.is_https(Some(ip("10.0.0.1")), &h));
    }
}
