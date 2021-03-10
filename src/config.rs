use std::net::SocketAddr;

#[derive(Default, Clone)]
pub struct Config {
    pub bootstrap: Option<Vec<SocketAddr>>,
    pub ephemeral: bool,
}

impl Config {
    pub fn set_bootstrap_nodes(mut self, nodes: Vec<SocketAddr>) -> Self {
        self.bootstrap = Some(nodes);
        self
    }

    pub fn set_ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;
        self
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TopicConfig {
    pub announce: bool,
    pub lookup: bool,
}

impl TopicConfig {
    pub fn both() -> Self {
        Self {
            announce: true,
            lookup: true,
        }
    }

    pub fn announce_and_lookup() -> Self {
        Self::both()
    }
}