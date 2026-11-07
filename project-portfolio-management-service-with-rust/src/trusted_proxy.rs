//! Which proxies are trusted to speak for the client. One list, `TRUSTED_PROXIES` (comma-separated IPs or CIDR ranges,
//! e.g. `10.0.0.0/8,::1`), which axa reads at boot: axa resolves the client address from `X-Forwarded-For` with it, and
//! this service believes `X-Forwarded-Proto` (for HSTS) only from the same peers. An unset list trusts nobody, so a
//! spoofed forwarding header from a direct client is ignored.
//!
//! `PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES`, the name this service read before axa did, still works alone, with
//! a warning. Set to a different list from `TRUSTED_PROXIES`, the service refuses to start, as there would be no
//! saying which was meant.

use axa::client::ProxyNet;
use axum::extract::ConnectInfo;
use axum::http::{Extensions, HeaderMap};
use std::net::{IpAddr, SocketAddr};

/// The one setting axa and this service read.
pub const TRUSTED_PROXIES: &str = "TRUSTED_PROXIES";

/// The name this service read before axa read the list too.
pub const OLD_TRUSTED_PROXIES: &str = "PROJECT_PORTFOLIO_MANAGEMENT_TRUSTED_PROXIES";

/// The socket peer of a request, when the server recorded one (the real
/// server does; an in-process test transport may not).
#[must_use]
pub fn peer_ip(extensions: &Extensions) -> Option<IpAddr> {
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}

/// The entries of a list, trimmed, with empty ones dropped.
fn entries(list: &str) -> impl Iterator<Item = &str> {
    list.split(',').map(str::trim).filter(|e| !e.is_empty())
}

/// What the old name says, when `TRUSTED_PROXIES` is unset and so must take it on. `lookup` reads one variable.
///
/// # Errors
///
/// Both names set, to lists with different entries.
pub fn old_setting(lookup: impl Fn(&str) -> Option<String>) -> Result<Option<String>, String> {
    let read = |name: &str| lookup(name).filter(|v| entries(v).next().is_some());
    let set = |list: &str| {
        entries(list)
            .map(str::to_string)
            .collect::<std::collections::BTreeSet<_>>()
    };
    match (read(TRUSTED_PROXIES), read(OLD_TRUSTED_PROXIES)) {
        (Some(new), Some(old)) => {
            if set(&new) == set(&old) {
                Ok(None)
            } else {
                Err(format!(
                    "refusing to start: {TRUSTED_PROXIES} and {OLD_TRUSTED_PROXIES} name different proxies; set {TRUSTED_PROXIES} alone"
                ))
            }
        }
        (None, old) => Ok(old),
        (Some(_), None) => Ok(None),
    }
}

/// The set of peers allowed to assert the scheme.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrustedProxies(Vec<ProxyNet>);

impl TrustedProxies {
    /// Trust nobody (the default).
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    /// The list axa resolved at boot (`ctx.config.http.trusted_proxies`), so both trust the same peers.
    #[must_use]
    pub fn from_nets(nets: Vec<ProxyNet>) -> Self {
        Self(nets)
    }

    /// Parse a comma-separated list of IPs / CIDR ranges, as axa's [`ProxyNet`] reads each.
    ///
    /// # Errors
    ///
    /// The first entry that is not a valid address or range.
    pub fn parse(list: &str) -> Result<Self, String> {
        entries(list)
            .map(|entry| {
                entry
                    .parse::<ProxyNet>()
                    .map_err(|error| format!("{entry}: {error}"))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }

    /// The parsed entries, for axa's configuration.
    #[must_use]
    pub fn into_nets(self) -> Vec<ProxyNet> {
        self.0
    }

    /// Whether `ip` is a trusted proxy.
    #[must_use]
    pub fn contains(&self, ip: IpAddr) -> bool {
        self.0.iter().any(|net| net.contains(ip))
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

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            vars.iter()
                .find(|(n, _)| *n == name)
                .map(|(_, v)| (*v).to_string())
        }
    }

    #[test]
    fn the_old_name_is_used_only_when_the_new_one_is_unset() {
        assert_eq!(old_setting(env(&[])), Ok(None));
        assert_eq!(old_setting(env(&[(TRUSTED_PROXIES, "10.0.0.1")])), Ok(None));
        assert_eq!(
            old_setting(env(&[(OLD_TRUSTED_PROXIES, "10.0.0.1")])),
            Ok(Some("10.0.0.1".to_string()))
        );
        // A blank new name is unset.
        assert_eq!(
            old_setting(env(&[
                (TRUSTED_PROXIES, " , "),
                (OLD_TRUSTED_PROXIES, "10.0.0.1")
            ])),
            Ok(Some("10.0.0.1".to_string()))
        );
        // The same entries, in any order or spacing, agree.
        assert_eq!(
            old_setting(env(&[
                (TRUSTED_PROXIES, "10.0.0.1,10.0.0.0/8"),
                (OLD_TRUSTED_PROXIES, " 10.0.0.0/8 , 10.0.0.1 ")
            ])),
            Ok(None)
        );
    }

    #[test]
    fn two_different_lists_refuse_to_start() {
        let refused = old_setting(env(&[
            (TRUSTED_PROXIES, "10.0.0.1"),
            (OLD_TRUSTED_PROXIES, "10.0.0.2"),
        ]))
        .unwrap_err();
        assert!(
            refused.contains(TRUSTED_PROXIES) && refused.contains(OLD_TRUSTED_PROXIES),
            "{refused}"
        );
    }

    #[test]
    fn the_list_resolves_clients_as_axa_does() {
        let nets = TrustedProxies::parse("10.0.0.0/8").unwrap().into_nets();
        let mut h = HeaderMap::new();
        h.insert(
            "x-forwarded-for",
            "6.6.6.6, 198.51.100.7, 10.0.0.2".parse().unwrap(),
        );
        // A trusted peer speaks for the right-most untrusted hop; an untrusted one only for itself.
        assert_eq!(
            axa::client::resolve(ip("10.0.0.1"), &h, &nets),
            ip("198.51.100.7")
        );
        assert_eq!(
            axa::client::resolve(ip("203.0.113.9"), &h, &nets),
            ip("203.0.113.9")
        );
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
