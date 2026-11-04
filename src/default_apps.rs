#[derive(Debug, Clone)]
pub struct DefaultApp {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub package: String,
}
impl DefaultApp {
    pub fn new(id: &str, name: &str, purpose: &str, package: &str) -> Self {
        Self { id: id.to_string(), name: name.to_string(), purpose: purpose.to_string(), package: package.to_string() }
    }
}

pub struct DefaultAppProfile { apps: Vec<DefaultApp> }
impl DefaultAppProfile {
    pub fn sigma_default() -> Self {
        let mut profile = Self { apps: Vec::new() };
        profile.apps.push(DefaultApp::new("browser", "Firefox", "web-browser", "firefox"));
        profile.apps.push(DefaultApp::new("files", "Nautilus", "file-manager", "nautilus"));
        profile.apps.push(DefaultApp::new("terminal", "GNOME Terminal", "terminal-emulator", "gnome-terminal"));
        profile.apps.push(DefaultApp::new("editor", "Gedit", "text-editor", "gedit"));
        profile.apps.push(DefaultApp::new("media", "Celluloid", "media-player", "celluloid"));
        profile.apps.push(DefaultApp::new("viewer", "Evince", "document-viewer", "evince"));
        profile.apps.push(DefaultApp::new("image-viewer", "Shotwell", "photo-tools", "shotwell"));
        profile.apps.push(DefaultApp::new("office", "LibreOffice", "office-suite", "libreoffice"));
        profile
    }
    pub fn count(&self) -> usize { self.apps.len() }
    pub fn find(&self, purpose: &str) -> Option<&DefaultApp> {
        self.apps.iter().find(|a| a.purpose == purpose)
    }
    pub fn all(&self) -> &[DefaultApp] { &self.apps }
}
impl Default for DefaultAppProfile { fn default() -> Self { Self::sigma_default() } }
