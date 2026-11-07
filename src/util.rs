//! docker model — Docker container management.

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub name: String,
    pub tag: String,
    pub size: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub containers: Vec<Container>,
    pub images: Vec<Image>,
}

impl Default for Model {
    fn default() -> Self { Self { containers: vec![], images: vec![
        Image { name: "alpine".into(), tag: "latest".into(), size: "5MB".into() },
    ] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn pull(&mut self, image: &str) -> bool {
        self.images.push(Image { name: image.into(), tag: "latest".into(), size: "unknown".into() });
        true
    }

    pub fn run(&mut self, image: &str) -> bool {
        let id = format!("{:x}", self.containers.len() + 1);
        self.containers.push(Container { id, image: image.into(), status: "running".into(), ports: vec![] });
        true
    }

    pub fn list(&self) -> &[Container] { &self.containers }
    pub fn images(&self) -> &[Image] { &self.images }

    pub fn stop(&mut self, id: &str) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.id == id) { c.status = "exited".into(); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pull_run_stop() {
        let mut d = Model::new();
        assert!(d.run("alpine"));
        assert!(d.stop("1"));
        assert_eq!(d.list()[0].status, "exited");
    }
}
