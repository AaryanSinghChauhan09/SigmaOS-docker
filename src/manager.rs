//! Docker manager.

use std::collections::HashMap;
use crate::container::{Container, ContainerState, ContainerConfig};
use crate::image::{Image, ImageState};
use crate::volume::{Volume, VolumeState};
use crate::network::{Network, NetworkState};
use crate::build::{BuildContext, BuildState};
use crate::compose::{ComposeFile, ComposeState};
use crate::swarm::{Swarm, SwarmState};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max containers
    pub max_containers: usize,
    /// Max images
    pub max_images: usize,
    /// Max volumes
    pub max_volumes: usize,
    /// Max networks
    pub max_networks: usize,
    /// Max compose files
    pub max_compose_files: usize,
    /// Default network
    pub default_network: String,
    /// Storage driver
    pub storage_driver: String,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_containers: 1000,
            max_images: 5000,
            max_volumes: 1000,
            max_networks: 100,
            max_compose_files: 100,
            default_network: "bridge".into(),
            storage_driver: "overlay2".into(),
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total containers
    pub total_containers: usize,
    /// Running containers
    pub running_containers: usize,
    /// Total images
    pub total_images: usize,
    /// Total volumes
    pub total_volumes: usize,
    /// Total networks
    pub total_networks: usize,
    /// Total compose files
    pub total_compose_files: usize,
    /// Total builds
    pub total_builds: usize,
    /// Swarm active
    pub swarm_active: bool,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Docker manager
#[derive(Debug, Clone)]
pub struct DockerManager {
    /// Containers
    containers: HashMap<String, Container>,
    /// Images
    images: HashMap<String, Image>,
    /// Volumes
    volumes: HashMap<String, Volume>,
    /// Networks
    networks: HashMap<String, Network>,
    /// Compose files
    compose_files: HashMap<String, ComposeFile>,
    /// Build contexts
    builds: HashMap<String, BuildContext>,
    /// Swarm
    swarm: Option<Swarm>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for DockerManager {
    fn default() -> Self {
        Self {
            containers: HashMap::new(),
            images: HashMap::new(),
            volumes: HashMap::new(),
            networks: HashMap::new(),
            compose_files: HashMap::new(),
            builds: HashMap::new(),
            swarm: None,
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_containers: 0,
                running_containers: 0,
                total_images: 0,
                total_volumes: 0,
                total_networks: 0,
                total_compose_files: 0,
                total_builds: 0,
                swarm_active: false,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl DockerManager {
    /// Create new docker manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            containers: HashMap::new(),
            images: HashMap::new(),
            volumes: HashMap::new(),
            networks: HashMap::new(),
            compose_files: HashMap::new(),
            builds: HashMap::new(),
            swarm: None,
            options,
            statistics: ManagerStatistics {
                total_containers: 0,
                running_containers: 0,
                total_images: 0,
                total_volumes: 0,
                total_networks: 0,
                total_compose_files: 0,
                total_builds: 0,
                swarm_active: false,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Create default network
        let network = Network::new(self.options.default_network.clone());
        self.networks.insert(network.id.clone(), network);
        self.update_statistics();
    }
    
    // Container methods
    
    /// Create container
    pub fn create_container(&mut self, name: String, image: String) -> String {
        let container = Container::with_image(name, image);
        let id = container.id.clone();
        
        self.containers.insert(id.clone(), container);
        self.update_statistics();
        id
    }
    
    /// Create container with config
    pub fn create_container_with_config(&mut self, name: String, image: String, config: ContainerConfig) -> String {
        let mut container = Container::with_image(name, image);
        container.set_config(config);
        let id = container.id.clone();
        
        self.containers.insert(id.clone(), container);
        self.update_statistics();
        id
    }
    
    /// Get container
    pub fn get_container(&self, container_id: &str) -> Option<&Container> {
        self.containers.get(container_id)
    }
    
    /// Get container mutably
    pub fn get_container_mut(&mut self, container_id: &str) -> Option<&mut Container> {
        self.containers.get_mut(container_id)
    }
    
    /// Get all containers
    pub fn list_containers(&self) -> Vec<&Container> {
        self.containers.values().collect()
    }
    
    /// Remove container
    pub fn remove_container(&mut self, container_id: &str) -> bool {
        // Remove from networks
        for network in self.networks.values_mut() {
            network.remove_container(container_id);
        }
        
        if self.containers.remove(container_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start container
    pub fn start_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.start();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop container
    pub fn stop_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.stop();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Restart container
    pub fn restart_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.restart();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Pause container
    pub fn pause_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.pause();
            return true;
        }
        false
    }
    
    /// Unpause container
    pub fn unpause_container(&mut self, container_id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.unpause();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Kill container
    pub fn kill_container(&mut self, container_id: &str, signal: i32) -> bool {
        if let Some(container) = self.containers.get_mut(container_id) {
            container.kill(signal);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get containers by state
    pub fn get_containers_by_state(&self, state: &ContainerState) -> Vec<&Container> {
        self.containers.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Get running containers
    pub fn get_running_containers(&self) -> Vec<&Container> {
        self.get_containers_by_state(&ContainerState::Running)
    }
    
    /// Search containers
    pub fn search_containers(&self, query: &str) -> Vec<&Container> {
        let query_lower = query.to_lowercase();
        self.containers.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Image methods
    
    /// Pull image
    pub fn pull_image(&mut self, name: String, tag: String) -> String {
        let full_name = format!("{}:{}", name, tag);
        let image = Image::new(full_name);
        let id = image.id.clone();
        
        self.images.insert(id.clone(), image);
        self.update_statistics();
        id
    }
    
    /// Get image
    pub fn get_image(&self, image_id: &str) -> Option<&Image> {
        self.images.get(image_id)
    }
    
    /// Get image mutably
    pub fn get_image_mut(&mut self, image_id: &str) -> Option<&mut Image> {
        self.images.get_mut(image_id)
    }
    
    /// Get all images
    pub fn list_images(&self) -> Vec<&Image> {
        self.images.values().collect()
    }
    
    /// Remove image
    pub fn remove_image(&mut self, image_id: &str) -> bool {
        if self.images.remove(image_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get images by state
    pub fn get_images_by_state(&self, state: &ImageState) -> Vec<&Image> {
        self.images.values()
            .filter(|i| i.state == *state)
            .collect()
    }
    
    /// Search images
    pub fn search_images(&self, query: &str) -> Vec<&Image> {
        let query_lower = query.to_lowercase();
        self.images.values()
            .filter(|i| i.name.to_lowercase().contains(&query_lower) ||
                       i.tag.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Volume methods
    
    /// Create volume
    pub fn create_volume(&mut self, name: String) -> String {
        let volume = Volume::new(name);
        let id = volume.id.clone();
        
        self.volumes.insert(id.clone(), volume);
        self.update_statistics();
        id
    }
    
    /// Get volume
    pub fn get_volume(&self, volume_id: &str) -> Option<&Volume> {
        self.volumes.get(volume_id)
    }
    
    /// Get volume mutably
    pub fn get_volume_mut(&mut self, volume_id: &str) -> Option<&mut Volume> {
        self.volumes.get_mut(volume_id)
    }
    
    /// Get all volumes
    pub fn list_volumes(&self) -> Vec<&Volume> {
        self.volumes.values().collect()
    }
    
    /// Remove volume
    pub fn remove_volume(&mut self, volume_id: &str) -> bool {
        if let Some(volume) = self.volumes.get(volume_id) {
            if volume.is_in_use() {
                return false;
            }
        }
        
        if self.volumes.remove(volume_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get volumes by state
    pub fn get_volumes_by_state(&self, state: &VolumeState) -> Vec<&Volume> {
        self.volumes.values()
            .filter(|v| v.state == *state)
            .collect()
    }
    
    /// Search volumes
    pub fn search_volumes(&self, query: &str) -> Vec<&Volume> {
        let query_lower = query.to_lowercase();
        self.volumes.values()
            .filter(|v| v.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Network methods
    
    /// Create network
    pub fn create_network(&mut self, name: String) -> String {
        let network = Network::new(name);
        let id = network.id.clone();
        
        self.networks.insert(id.clone(), network);
        self.update_statistics();
        id
    }
    
    /// Get network
    pub fn get_network(&self, network_id: &str) -> Option<&Network> {
        self.networks.get(network_id)
    }
    
    /// Get network mutably
    pub fn get_network_mut(&mut self, network_id: &str) -> Option<&mut Network> {
        self.networks.get_mut(network_id)
    }
    
    /// Get all networks
    pub fn list_networks(&self) -> Vec<&Network> {
        self.networks.values().collect()
    }
    
    /// Remove network
    pub fn remove_network(&mut self, network_id: &str) -> bool {
        if self.networks.remove(network_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Connect container to network
    pub fn connect_container_to_network(&mut self, container_id: &str, network_id: &str) -> bool {
        if let Some(network) = self.networks.get_mut(network_id) {
            network.add_container(container_id.to_string());
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disconnect container from network
    pub fn disconnect_container_from_network(&mut self, container_id: &str, network_id: &str) -> bool {
        if let Some(network) = self.networks.get_mut(network_id) {
            if network.remove_container(container_id) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get networks by state
    pub fn get_networks_by_state(&self, state: &NetworkState) -> Vec<&Network> {
        self.networks.values()
            .filter(|n| n.state == *state)
            .collect()
    }
    
    /// Get active networks
    pub fn get_active_networks(&self) -> Vec<&Network> {
        self.get_networks_by_state(&NetworkState::Active)
    }
    
    /// Search networks
    pub fn search_networks(&self, query: &str) -> Vec<&Network> {
        let query_lower = query.to_lowercase();
        self.networks.values()
            .filter(|n| n.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Compose methods
    
    /// Create compose file
    pub fn create_compose_file(&mut self, name: String) -> String {
        let compose = ComposeFile::new(name.clone());
        let id = format!("compose-{}", self.compose_files.len() + 1);
        
        let mut compose = compose;
        compose.path = name;
        
        self.compose_files.insert(id.clone(), compose);
        self.update_statistics();
        id
    }
    
    /// Get compose file
    pub fn get_compose_file(&self, compose_id: &str) -> Option<&ComposeFile> {
        self.compose_files.get(compose_id)
    }
    
    /// Get compose file mutably
    pub fn get_compose_file_mut(&mut self, compose_id: &str) -> Option<&mut ComposeFile> {
        self.compose_files.get_mut(compose_id)
    }
    
    /// Get all compose files
    pub fn list_compose_files(&self) -> Vec<&ComposeFile> {
        self.compose_files.values().collect()
    }
    
    /// Remove compose file
    pub fn remove_compose_file(&mut self, compose_id: &str) -> bool {
        if self.compose_files.remove(compose_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start compose file
    pub fn start_compose(&mut self, compose_id: &str) -> bool {
        if let Some(compose) = self.compose_files.get_mut(compose_id) {
            compose.start();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Stop compose file
    pub fn stop_compose(&mut self, compose_id: &str) -> bool {
        if let Some(compose) = self.compose_files.get_mut(compose_id) {
            compose.stop();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get compose files by state
    pub fn get_compose_files_by_state(&self, state: &ComposeState) -> Vec<&ComposeFile> {
        self.compose_files.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Search compose files
    pub fn search_compose_files(&self, query: &str) -> Vec<&ComposeFile> {
        let query_lower = query.to_lowercase();
        self.compose_files.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower) ||
                       c.path.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Build methods
    
    /// Create build context
    pub fn create_build(&mut self, path: String) -> String {
        let build = BuildContext::new(path);
        let id = format!("build-{}", self.builds.len() + 1);
        
        self.builds.insert(id.clone(), build);
        self.update_statistics();
        id
    }
    
    /// Get build
    pub fn get_build(&self, build_id: &str) -> Option<&BuildContext> {
        self.builds.get(build_id)
    }
    
    /// Get build mutably
    pub fn get_build_mut(&mut self, build_id: &str) -> Option<&mut BuildContext> {
        self.builds.get_mut(build_id)
    }
    
    /// Get all builds
    pub fn list_builds(&self) -> Vec<&BuildContext> {
        self.builds.values().collect()
    }
    
    /// Remove build
    pub fn remove_build(&mut self, build_id: &str) -> bool {
        if self.builds.remove(build_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start build
    pub fn start_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            build.start_build();
            return true;
        }
        false
    }
    
    /// Complete build
    pub fn complete_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            build.complete_build();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Fail build
    pub fn fail_build(&mut self, build_id: &str) -> bool {
        if let Some(build) = self.builds.get_mut(build_id) {
            build.fail_build();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get builds by state
    pub fn get_builds_by_state(&self, state: &BuildState) -> Vec<&BuildContext> {
        self.builds.values()
            .filter(|b| b.state == *state)
            .collect()
    }
    
    /// Search builds
    pub fn search_builds(&self, query: &str) -> Vec<&BuildContext> {
        let query_lower = query.to_lowercase();
        self.builds.values()
            .filter(|b| b.path.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Swarm methods
    
    /// Initialize swarm
    pub fn init_swarm(&mut self) {
        let mut swarm = Swarm::new();
        swarm.activate();
        // Add manager node
        let mut manager_node = crate::swarm::SwarmNode::new("manager".into());
        manager_node.set_role(crate::swarm::NodeRole::Manager);
        swarm.add_node(manager_node);
        self.swarm = Some(swarm);
        self.update_statistics();
    }
    
    /// Leave swarm
    pub fn leave_swarm(&mut self) {
        self.swarm = None;
        self.update_statistics();
    }
    
    /// Get swarm
    pub fn get_swarm(&self) -> Option<&Swarm> {
        self.swarm.as_ref()
    }
    
    /// Get swarm mutably
    pub fn get_swarm_mut(&mut self) -> Option<&mut Swarm> {
        self.swarm.as_mut()
    }
    
    /// Add node to swarm
    pub fn add_swarm_node(&mut self, node: crate::swarm::SwarmNode) -> bool {
        if let Some(swarm) = &mut self.swarm {
            if swarm.add_node(node) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Remove node from swarm
    pub fn remove_swarm_node(&mut self, node_id: &str) -> bool {
        if let Some(swarm) = &mut self.swarm {
            if swarm.remove_node(node_id) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Add service to swarm
    pub fn add_swarm_service(&mut self, service: crate::swarm::SwarmService) -> bool {
        if let Some(swarm) = &mut self.swarm {
            if swarm.add_service(service) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Remove service from swarm
    pub fn remove_swarm_service(&mut self, service_id: &str) -> bool {
        if let Some(swarm) = &mut self.swarm {
            if swarm.remove_service(service_id) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_containers = self.containers.len();
        self.statistics.running_containers = self.containers.values()
            .filter(|c| c.is_running())
            .count();
        self.statistics.total_images = self.images.len();
        self.statistics.total_volumes = self.volumes.len();
        self.statistics.total_networks = self.networks.len();
        self.statistics.total_compose_files = self.compose_files.len();
        self.statistics.total_builds = self.builds.len();
        self.statistics.swarm_active = self.swarm.as_ref()
            .map(|s| s.is_active())
            .unwrap_or(false);
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            containers: self.containers.values().cloned().collect(),
            images: self.images.values().cloned().collect(),
            volumes: self.volumes.values().cloned().collect(),
            networks: self.networks.values().cloned().collect(),
            compose_files: self.compose_files.values().cloned().collect(),
            builds: self.builds.values().cloned().collect(),
            swarm: self.swarm.clone(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.containers.clear();
        for container in export.containers {
            self.containers.insert(container.id.clone(), container);
        }
        
        self.images.clear();
        for image in export.images {
            self.images.insert(image.id.clone(), image);
        }
        
        self.volumes.clear();
        for volume in export.volumes {
            self.volumes.insert(volume.id.clone(), volume);
        }
        
        self.networks.clear();
        for network in export.networks {
            self.networks.insert(network.id.clone(), network);
        }
        
        self.compose_files.clear();
        for compose in export.compose_files {
            self.compose_files.insert(compose.name.clone(), compose);
        }
        
        self.builds.clear();
        for build in export.builds {
            self.builds.insert(build.path.clone(), build);
        }
        
        self.swarm = export.swarm;
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Containers
    pub containers: Vec<Container>,
    /// Images
    pub images: Vec<Image>,
    /// Volumes
    pub volumes: Vec<Volume>,
    /// Networks
    pub networks: Vec<Network>,
    /// Compose files
    pub compose_files: Vec<ComposeFile>,
    /// Builds
    pub builds: Vec<BuildContext>,
    /// Swarm
    pub swarm: Option<Swarm>,
}
