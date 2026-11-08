//! Docker Swarm definitions.

/// Swarm state
#[derive(Debug, Clone, PartialEq)]
pub enum SwarmState {
    /// Inactive swarm
    Inactive,
    /// Active swarm
    Active,
    /// Locked swarm
    Locked,
}

/// Swarm node
#[derive(Debug, Clone)]
pub struct SwarmNode {
    /// Node ID
    pub id: String,
    /// Node name
    pub name: String,
    /// Node role (manager/worker)
    pub role: NodeRole,
    /// Node status
    pub status: NodeStatus,
    /// Node address
    pub address: String,
    /// Node labels
    pub labels: Vec<(String, String)>,
    /// Created timestamp
    pub created_at: u64,
}

/// Node role
#[derive(Debug, Clone, PartialEq)]
pub enum NodeRole {
    /// Manager node
    Manager,
    /// Worker node
    Worker,
}

/// Node status
#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    /// Active node
    Active,
    /// Down node
    Down,
    /// Drain node
    Drain,
    /// Pause node
    Pause,
}

impl SwarmNode {
    /// Create new swarm node
    pub fn new(name: String) -> Self {
        Self {
            id: format!("node-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            role: NodeRole::Worker,
            status: NodeStatus::Active,
            address: String::new(),
            labels: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Set role
    pub fn set_role(&mut self, role: NodeRole) {
        self.role = role;
    }
    
    /// Set status
    pub fn set_status(&mut self, status: NodeStatus) {
        self.status = status;
    }
    
    /// Add label
    pub fn add_label(&mut self, key: String, value: String) {
        self.labels.push((key, value));
    }
    
    /// Check if manager
    pub fn is_manager(&self) -> bool {
        self.role == NodeRole::Manager
    }
    
    /// Check if worker
    pub fn is_worker(&self) -> bool {
        self.role == NodeRole::Worker
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.status == NodeStatus::Active
    }
    
    /// Get node age in seconds
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

/// Service state
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    /// New service
    New,
    /// Pending service
    Pending,
    /// Assigned service
    Assigned,
    /// Accepted service
    Accepted,
    /// Preparing service
    Preparing,
    /// Starting service
    Starting,
    /// Running service
    Running,
    /// Complete service
    Complete,
    /// Shutdown service
    Shutdown,
    /// Failed service
    Failed,
    /// Rejected service
    Rejected,
    /// Remove service
    Remove,
}

/// Swarm service
#[derive(Debug, Clone)]
pub struct SwarmService {
    /// Service ID
    pub id: String,
    /// Service name
    pub name: String,
    /// Service image
    pub image: String,
    /// Service state
    pub state: ServiceState,
    /// Service replicas
    pub replicas: u32,
    /// Service ports
    pub ports: Vec<(u16, u16)>,
    /// Service env vars
    pub env_vars: Vec<(String, String)>,
    /// Service labels
    pub labels: Vec<(String, String)>,
    /// Created timestamp
    pub created_at: u64,
}

impl SwarmService {
    /// Create new swarm service
    pub fn new(name: String, image: String) -> Self {
        Self {
            id: format!("service-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            image,
            state: ServiceState::New,
            replicas: 1,
            ports: vec![],
            env_vars: vec![],
            labels: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Set replicas
    pub fn set_replicas(&mut self, replicas: u32) {
        self.replicas = replicas;
    }
    
    /// Add port
    pub fn add_port(&mut self, host: u16, container: u16) {
        self.ports.push((host, container));
    }
    
    /// Add env var
    pub fn add_env(&mut self, key: String, value: String) {
        self.env_vars.push((key, value));
    }
    
    /// Add label
    pub fn add_label(&mut self, key: String, value: String) {
        self.labels.push((key, value));
    }
    
    /// Start service
    pub fn start(&mut self) {
        self.state = ServiceState::Running;
    }
    
    /// Stop service
    pub fn stop(&mut self) {
        self.state = ServiceState::Shutdown;
    }
    
    /// Fail service
    pub fn fail(&mut self) {
        self.state = ServiceState::Failed;
    }
    
    /// Check if running
    pub fn is_running(&self) -> bool {
        self.state == ServiceState::Running
    }
    
    /// Check if failed
    pub fn is_failed(&self) -> bool {
        self.state == ServiceState::Failed
    }
    
    /// Get service age in seconds
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

/// Swarm
#[derive(Debug, Clone)]
pub struct Swarm {
    /// Swarm ID
    pub id: String,
    /// Swarm name
    pub name: String,
    /// Swarm state
    pub state: SwarmState,
    /// Swarm nodes
    pub nodes: Vec<SwarmNode>,
    /// Swarm services
    pub services: Vec<SwarmService>,
    /// Swarm created timestamp
    pub created_at: u64,
}

impl Swarm {
    /// Create new swarm
    pub fn new() -> Self {
        Self {
            id: format!("swarm-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name: "default".into(),
            state: SwarmState::Inactive,
            nodes: vec![],
            services: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Activate swarm
    pub fn activate(&mut self) {
        self.state = SwarmState::Active;
    }
    
    /// Deactivate swarm
    pub fn deactivate(&mut self) {
        self.state = SwarmState::Inactive;
    }
    
    /// Add node
    pub fn add_node(&mut self, node: SwarmNode) -> bool {
        if self.nodes.iter().any(|n| n.id == node.id) {
            return false;
        }
        self.nodes.push(node);
        true
    }
    
    /// Remove node
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == node_id) {
            self.nodes.remove(pos);
            return true;
        }
        false
    }
    
    /// Get node
    pub fn get_node(&self, node_id: &str) -> Option<&SwarmNode> {
        self.nodes.iter().find(|n| n.id == node_id)
    }
    
    /// Add service
    pub fn add_service(&mut self, service: SwarmService) -> bool {
        if self.services.iter().any(|s| s.id == service.id) {
            return false;
        }
        self.services.push(service);
        true
    }
    
    /// Remove service
    pub fn remove_service(&mut self, service_id: &str) -> bool {
        if let Some(pos) = self.services.iter().position(|s| s.id == service_id) {
            self.services.remove(pos);
            return true;
        }
        false
    }
    
    /// Get service
    pub fn get_service(&self, service_id: &str) -> Option<&SwarmService> {
        self.services.iter().find(|s| s.id == service_id)
    }
    
    /// Get node count
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Get service count
    pub fn get_service_count(&self) -> usize {
        self.services.len()
    }
    
    /// Get manager count
    pub fn get_manager_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_manager()).count()
    }
    
    /// Get worker count
    pub fn get_worker_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_worker()).count()
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == SwarmState::Active
    }
    
    /// Get swarm age in seconds
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

impl Default for Swarm {
    fn default() -> Self {
        Self::new()
    }
}
