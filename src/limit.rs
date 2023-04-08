use reqwest::{Client, Request};
use std::collections::VecDeque;

pub struct Limit {
    limit: i64,
    remaining: i64,
    reset: i64,
    bucket: String,
}

pub struct LimitedRequester {
    http: Client,
    limit: Vec<Limit>,
    requests: VecDeque<Request>,
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    pub fn new(api_url: String) -> Self {
        LimitedRequester {
            limit: LimitedRequester::check_limits(api_url),
            http: Client::new(),
            requests: VecDeque::new(),
        }
    }

    pub fn check_limits(url: String) -> Vec<Limit> {
        let client = Client::new();
        let url_parsed = crate::URLBundle::parse_url(url) + "/api/policies/instance/limits";
        let mut limit_vector = Vec::new();
        limit_vector.push(Limit {
            limit: -1,
            remaining: -1,
            reset: -1,
            bucket: String::new(),
        }); // TODO: Implement
        limit_vector
    }
}

/* #[cfg(test)]  Tests work here as well, neat!
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        assert_eq!(1, 1)
    }
} */
