//! SigmaOS-docker — Docker engine component.
//!
//! Models Docker's container platform: containers, images, volumes,
//! networks, build system, compose stacks, and swarm services for
//! the SigmaOS desktop.

pub mod container;
pub mod image;
pub mod volume;
pub mod network;
pub mod build;
pub mod compose;
pub mod swarm;
pub mod manager;
pub mod options;

pub use container::{Container, ContainerState, ContainerConfig, ContainerStats};
pub use image::{Image, ImageConfig, ImageLayer, ImageState};
pub use volume::{Volume, VolumeConfig, VolumeState};
pub use network::{Network, NetworkConfig, NetworkDriver, NetworkState};
pub use build::{BuildContext, BuildStep, BuildResult, BuildState};
pub use compose::{ComposeFile, ComposeService, ComposeState};
pub use swarm::{Swarm, SwarmService, ServiceState, SwarmNode};
pub use manager::{DockerManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_container_state() {
        let states = vec![
            ContainerState::Created,
            ContainerState::Running,
            ContainerState::Paused,
            ContainerState::Restarting,
            ContainerState::Removing,
            ContainerState::Exited,
            ContainerState::Dead,
        ];
        assert_eq!(states.len(), 7);
    }
    
    #[test]
    fn test_image_state() {
        let states = vec![
            ImageState::Available,
            ImageState::Pulling,
            ImageState::Building,
            ImageState::Removing,
        ];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_volume_state() {
        let states = vec![
            VolumeState::Available,
            VolumeState::InUse,
            VolumeState::Removing,
        ];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_network_state() {
        let states = vec![
            NetworkState::Active,
            NetworkState::Inactive,
            NetworkState::Removing,
        ];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_build_state() {
        let states = vec![
            BuildState::Pending,
            BuildState::Building,
            BuildState::Completed,
            BuildState::Failed,
            BuildState::Cancelled,
        ];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_compose_state() {
        let states = vec![
            ComposeState::Created,
            ComposeState::Starting,
            ComposeState::Running,
            ComposeState::Stopping,
            ComposeState::Stopped,
        ];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_service_state() {
        let states = vec![
            ServiceState::New,
            ServiceState::Pending,
            ServiceState::Assigned,
            ServiceState::Accepted,
            ServiceState::Preparing,
            ServiceState::Starting,
            ServiceState::Running,
            ServiceState::Complete,
            ServiceState::Shutdown,
            ServiceState::Failed,
            ServiceState::Rejected,
            ServiceState::Remove,
        ];
        assert_eq!(states.len(), 12);
    }
    
    #[test]
    fn test_container_creation() {
        let container = Container::new("web".into());
        assert_eq!(container.name, "web");
        assert_eq!(container.state, ContainerState::Created);
        assert!(container.image.is_none());
    }
    
    #[test]
    fn test_container_with_image() {
        let container = Container::with_image("web".into(), "nginx:latest".into());
        assert_eq!(container.image, Some("nginx:latest".into()));
    }
    
    #[test]
    fn test_container_config() {
        let mut config = ContainerConfig::default();
        config.ports = vec![(8080, 80)];
        config.env_vars = vec![("DEBUG".into(), "true".into())];
        config.memory_limit_mb = 512;
        config.restart_policy = "always".into();
        
        assert_eq!(config.ports.len(), 1);
        assert_eq!(config.restart_policy, "always");
    }
    
    #[test]
    fn test_container_lifecycle() {
        let mut container = Container::new("app".into());
        container.start();
        assert_eq!(container.state, ContainerState::Running);
        container.restart();
        assert_eq!(container.state, ContainerState::Running);
        container.stop();
        assert_eq!(container.state, ContainerState::Exited);
    }
    
    #[test]
    fn test_container_stats() {
        let mut container = Container::new("app".into());
        container.start();
        container.update_stats(1024, 2048, 10, 10.0);
        
        assert_eq!(container.stats.memory_usage_mb, Some(1024));
        assert_eq!(container.stats.cpu_usage_percent, Some(10.0));
    }
    
    #[test]
    fn test_image_creation() {
        let image = Image::new("nginx:latest".into());
        assert_eq!(image.get_full_name(), "nginx:latest");
        assert_eq!(image.state, ImageState::Available);
        assert!(image.layers.is_empty());
    }
    
    #[test]
    fn test_image_layer() {
        let layer = ImageLayer::new("sha256:abc123".into(), 1024 * 1024);
        assert_eq!(layer.digest, "sha256:abc123");
        assert_eq!(layer.size_bytes, 1024 * 1024);
    }
    
    #[test]
    fn test_volume_creation() {
        let volume = Volume::new("data".into());
        assert_eq!(volume.name, "data");
        assert_eq!(volume.state, VolumeState::Available);
    }
    
    #[test]
    fn test_network_creation() {
        let network = Network::new("bridge0".into());
        assert_eq!(network.name, "bridge0");
        assert_eq!(network.state, NetworkState::Active);
        assert_eq!(network.config.driver, NetworkDriver::Bridge);
    }
    
    #[test]
    fn test_compose_file() {
        let mut compose = ComposeFile::new("docker-compose.yml".into());
        compose.add_service(ComposeService::new("web".into()));
        compose.add_service(ComposeService::new("db".into()));
        
        assert_eq!(compose.services.len(), 2);
    }
    
    #[test]
    fn test_swarm_creation() {
        let swarm = Swarm::new();
        assert!(swarm.nodes.is_empty());
        assert!(swarm.services.is_empty());
        assert_eq!(swarm.state, crate::swarm::SwarmState::Inactive);
    }
    
    #[test]
    fn test_swarm_add_node() {
        let mut swarm = Swarm::new();
        swarm.add_node(SwarmNode::new("node1".into()));
        assert_eq!(swarm.nodes.len(), 1);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = DockerManager::new();
        assert!(manager.list_containers().is_empty());
        assert!(manager.list_images().is_empty());
        assert!(manager.list_compose_files().is_empty());
    }
    
    #[test]
    fn test_create_container() {
        let mut manager = DockerManager::new();
        let container_id = manager.create_container("web".into(), "nginx:latest".into());
        
        assert!(manager.get_container(&container_id).is_some());
        assert_eq!(manager.list_containers().len(), 1);
    }
    
    #[test]
    fn test_manager_test_container_lifecycle() {
        let mut manager = DockerManager::new();
        let container_id = manager.create_container("app".into(), "alpine".into());
        
        assert!(manager.start_container(&container_id));
        assert!(manager.restart_container(&container_id));
        assert!(manager.stop_container(&container_id));
    }
    
    #[test]
    fn test_remove_container() {
        let mut manager = DockerManager::new();
        let container_id = manager.create_container("app".into(), "alpine".into());
        manager.start_container(&container_id);
        manager.stop_container(&container_id);
        
        assert!(manager.remove_container(&container_id));
        assert!(manager.get_container(&container_id).is_none());
    }
    
    #[test]
    fn test_pull_image() {
        let mut manager = DockerManager::new();
        let image_id = manager.pull_image("nginx".into(), "latest".into());
        
        assert!(manager.get_image(&image_id).is_some());
        assert_eq!(manager.list_images().len(), 1);
    }
    
    #[test]
    fn test_create_volume() {
        let mut manager = DockerManager::new();
        let volume_id = manager.create_volume("data".into());
        
        assert!(manager.get_volume(&volume_id).is_some());
        assert_eq!(manager.list_volumes().len(), 1);
    }
    
    #[test]
    fn test_create_network() {
        let mut manager = DockerManager::new();
        let network_id = manager.create_network("mynet".into());
        
        assert!(manager.get_network(&network_id).is_some());
        assert_eq!(manager.list_networks().len(), 1);
    }
    
    #[test]
    fn test_manager_test_compose_file() {
        let mut manager = DockerManager::new();
        let compose_id = manager.create_compose_file("docker-compose.yml".into());
        
        assert!(manager.get_compose_file(&compose_id).is_some());
        assert_eq!(manager.list_compose_files().len(), 1);
    }
    
    #[test]
    fn test_swarm_management() {
        let mut manager = DockerManager::new();
        manager.init_swarm();
        assert!(manager.get_swarm().is_some());
        
        manager.leave_swarm();
        assert!(manager.get_swarm().is_none());
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = DockerManager::new();
        manager.create_container("app".into(), "alpine".into());
        manager.pull_image("nginx".into(), "latest".into());
        manager.create_volume("data".into());
        
        let export = manager.export_data();
        
        let mut manager2 = DockerManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_containers().len(), 1);
        assert_eq!(manager2.list_images().len(), 1);
        assert_eq!(manager2.list_volumes().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = DockerManager::new();
        manager.create_container("app1".into(), "alpine".into());
        manager.create_container("app2".into(), "nginx".into());
        manager.pull_image("redis".into(), "latest".into());
        manager.create_volume("data".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_containers, 2);
        assert_eq!(stats.total_images, 1);
        assert_eq!(stats.total_volumes, 1);
    }
}
