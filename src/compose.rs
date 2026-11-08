//! Docker Compose definitions.

use crate::container::ContainerConfig;

/// Compose state
#[derive(Debug, Clone, PartialEq)]
pub enum ComposeState {
    /// Created compose
    Created,
    /// Starting compose
    Starting,
    /// Running compose
    Running,
    /// Stopping compose
    Stopping,
    /// Stopped compose
    Stopped,
}

/// Compose service
#[derive(Debug, Clone)]
pub struct ComposeService {
    /// Service name
    pub name: String,
    /// Service image
    pub image: Option<String>,
    /// Service build context
    pub build: Option<String>,
    /// Service ports
    pub ports: Vec<(u16, u16)>,
    /// Service env vars
    pub env_vars: Vec<(String, String)>,
    /// Service volumes
    pub volumes: Vec<(String, String)>,
    /// Service depends_on
    pub depends_on: Vec<String>,
    /// Service command
    pub command: Option<String>,
    /// Service replicas
    pub replicas: u32,
    /// Container ID
    pub container_id: Option<String>,
}

impl ComposeService {
    /// Create new compose service
    pub fn new(name: String) -> Self {
        Self {
            name,
            image: None,
            build: None,
            ports: vec![],
            env_vars: vec![],
            volumes: vec![],
            depends_on: vec![],
            command: None,
            replicas: 1,
            container_id: None,
        }
    }
    
    /// Set image
    pub fn set_image(&mut self, image: String) {
        self.image = Some(image);
    }
    
    /// Set build
    pub fn set_build(&mut self, context: String) {
        self.build = Some(context);
    }
    
    /// Add port
    pub fn add_port(&mut self, host: u16, container: u16) {
        self.ports.push((host, container));
    }
    
    /// Add env var
    pub fn add_env(&mut self, key: String, value: String) {
        self.env_vars.push((key, value));
    }
    
    /// Add volume
    pub fn add_volume(&mut self, host: String, container: String) {
        self.volumes.push((host, container));
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, service: String) {
        if !self.depends_on.contains(&service) {
            self.depends_on.push(service);
        }
    }
    
    /// Set replicas
    pub fn set_replicas(&mut self, replicas: u32) {
        self.replicas = replicas;
    }
    
    /// Set container ID
    pub fn set_container_id(&mut self, id: String) {
        self.container_id = Some(id);
    }
    
    /// Convert to container config
    pub fn to_container_config(&self) -> ContainerConfig {
        ContainerConfig {
            ports: self.ports.clone(),
            env_vars: self.env_vars.clone(),
            volumes: self.volumes.iter()
                .map(|(h, c)| (h.clone(), c.clone(), false))
                .collect(),
            network: None,
            memory_limit_mb: 0,
            cpu_limit: 0.0,
            restart_policy: "no".into(),
            workdir: "/".into(),
            command: self.command.as_ref()
                .map(|c| vec!["/bin/sh".into(), "-c".into(), c.clone()])
                .unwrap_or_default(),
            entrypoint: None,
            labels: vec![],
            hostname: None,
            privileged: false,
        }
    }
}

/// Compose file
#[derive(Debug, Clone)]
pub struct ComposeFile {
    /// File name
    pub name: String,
    /// File path
    pub path: String,
    /// Compose services
    pub services: Vec<ComposeService>,
    /// Compose state
    pub state: ComposeState,
    /// Compose version
    pub version: String,
    /// Created timestamp
    pub created_at: u64,
}

impl ComposeFile {
    /// Create new compose file
    pub fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            path: name,
            services: vec![],
            state: ComposeState::Created,
            version: "3.8".into(),
            created_at: Self::now(),
        }
    }
    
    /// Add service
    pub fn add_service(&mut self, service: ComposeService) {
        self.services.push(service);
    }
    
    /// Get service
    pub fn get_service(&self, name: &str) -> Option<&ComposeService> {
        self.services.iter().find(|s| s.name == name)
    }
    
    /// Get service mutably
    pub fn get_service_mut(&mut self, name: &str) -> Option<&mut ComposeService> {
        self.services.iter_mut().find(|s| s.name == name)
    }
    
    /// Remove service
    pub fn remove_service(&mut self, name: &str) -> bool {
        if let Some(pos) = self.services.iter().position(|s| s.name == name) {
            self.services.remove(pos);
            return true;
        }
        false
    }
    
    /// Start compose
    pub fn start(&mut self) {
        self.state = ComposeState::Starting;
        self.state = ComposeState::Running;
    }
    
    /// Stop compose
    pub fn stop(&mut self) {
        self.state = ComposeState::Stopping;
        self.state = ComposeState::Stopped;
    }
    
    /// Get service count
    pub fn get_service_count(&self) -> usize {
        self.services.len()
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == ComposeState::Running
    }
    
    /// Check if stopped
    pub fn is_stopped(&self) -> bool {
        self.state == ComposeState::Stopped
    }
    
    /// Get total replicas
    pub fn get_total_replicas(&self) -> u32 {
        self.services.iter().map(|s| s.replicas).sum()
    }
    
    /// Get compose age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
