use crate::arch::Arch;
use crate::log_level::LogLevel;
use crate::path_ext::PathExt;
use crate::version_file_strategy::VersionFileStrategy;
use etcetera::BaseStrategy;
use url::Url;

#[derive(clap::Parser, Debug)]
pub struct FnmConfig {
    /// https://nodejs.org/dist/ mirror
    #[clap(
        long,
        env = "FNM_NODE_DIST_MIRROR",
        default_value = "https://nodejs.org/dist",
        global = true,
        hide_env_values = true
    )]
    pub node_dist_mirror: Url,

    /// The root directory of fnm installations.
    #[clap(
        long = "fnm-dir",
        env = "FNM_DIR",
        global = true,
        hide_env_values = true
    )]
    pub base_dir: Option<std::path::PathBuf>,

    /// Where the current node version link is stored.
    /// This value will be populated automatically by evaluating
    /// `fnm env` in your shell profile. Read more about it using `fnm help env`
    #[clap(long, env = "FNM_MULTISHELL_PATH", hide_env_values = true, hide = true)]
    multishell_path: Option<std::path::PathBuf>,

    /// The log level of fnm commands
    #[clap(
        long,
        env = "FNM_LOGLEVEL",
        default_value = "info",
        global = true,
        hide_env_values = true,
        possible_values = LogLevel::possible_values()
    )]
    log_level: LogLevel,

    /// Override the architecture of the installed Node binary.
    /// Defaults to arch of fnm binary.
    #[clap(
        long,
        env = "FNM_ARCH",
        default_value_t,
        global = true,
        hide_env_values = true,
        hide_default_value = true
    )]
    pub arch: Arch,

    /// A strategy for how to resolve the Node version. Used whenever `fnm use` or `fnm install` is
    /// called without a version, or when `--use-on-cd` is configured on evaluation.
    ///
    /// * `local`: Use the local version of Node defined within the current directory
    ///
    /// * `recursive`: Use the version of Node defined within the current directory and all parent directories
    #[clap(
        long,
        env = "FNM_VERSION_FILE_STRATEGY",
        possible_values = VersionFileStrategy::possible_values(),
        default_value = "local",
        global = true,
        hide_env_values = true,
    )]
    version_file_strategy: VersionFileStrategy,
}

impl Default for FnmConfig {
    fn default() -> Self {
        Self {
            node_dist_mirror: Url::parse("https://nodejs.org/dist/").unwrap(),
            base_dir: None,
            multishell_path: None,
            log_level: LogLevel::Info,
            arch: Arch::default(),
            version_file_strategy: VersionFileStrategy::default(),
        }
    }
}

impl FnmConfig {
    pub fn version_file_strategy(&self) -> &VersionFileStrategy {
        &self.version_file_strategy
    }

    pub fn multishell_path(&self) -> Option<&std::path::Path> {
        match &self.multishell_path {
            None => None,
            Some(v) => Some(v.as_path()),
        }
    }

    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    pub fn base_dir_with_default(&self) -> std::path::PathBuf {
        let user_pref = self.base_dir.clone();
        if let Some(dir) = user_pref {
            return dir;
        }

        let basedirs = etcetera::choose_base_strategy().expect("Can't get home directory");

        let modern = basedirs.data_dir().join("fnm");
        let legacy = basedirs.home_dir().join(".fnm");

        if modern.exists() {
            return modern;
        }
        if legacy.exists() {
            return legacy;
        }

        #[cfg(target_os = "macos")]
        {
            let basedirs = etcetera::base_strategy::Apple::new().expect("Can't get home directory");
            let legacy = basedirs.data_dir().join("fnm");
            if legacy.exists() {
                return legacy;
            }
        }

        modern.ensure_exists_silently()
    }

    pub fn installations_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("node-versions")
            .ensure_exists_silently()
    }

    pub fn default_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("aliases")
            .ensure_exists_silently()
    }

    #[cfg(test)]
    pub fn with_base_dir(mut self, base_dir: Option<std::path::PathBuf>) -> Self {
        self.base_dir = base_dir;
        self
    }
}
