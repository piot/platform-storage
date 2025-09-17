use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub type Result<T> = std::result::Result<T, io::Error>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaseDirs {
    /// Lowercased company/app for Windows & Linux
    company: String,
    app: String,
    /// Lowercased reverse-DNS for macOS, e.g. "`net.company_name.cool_app`"
    bundle_id: String,
}

impl BaseDirs {
    #[must_use] pub fn new(company: &str, app: &str, bundle_id: &str) -> Self {
        Self {
            company: company.to_lowercase(),
            app: app.to_lowercase(),
            bundle_id: bundle_id.to_lowercase(),
        }
    }

    /* ---------------- Purpose-based dirs ---------------- */

    /// Settings (device-local)
    /// - Windows: %LOCALAPPDATA%/company/app/settings/
    /// - macOS:   ~/Library/Application Support/<bundle_id>/settings/
    /// - Linux:   $XDG_CONFIG_HOME/company/app/  (fallback ~/.config/company/app/)
    #[must_use] pub fn settings_dir(&self) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            app_path(
                &win_localappdata_base(),
                &self.company,
                &self.app,
                Some("settings"),
            )
        }
        #[cfg(target_os = "macos")]
        {
            bundle_path(&mac_app_support_base(), &self.bundle_id, Some("settings"))
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            app_path(&xdg_config_home_base(), &self.company, &self.app, None)
        }
    }

    /// Saves (synced/backed up)
    /// - Windows: %USERPROFILE%/Saved Games/company/app/
    /// - macOS:   ~/Library/Application Support/<bundle_id>/saves/
    /// - Linux:   $XDG_DATA_HOME/company/app/  (fallback ~/.local/share/company/app/saves/)
    #[must_use] pub fn saves_dir(&self) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            app_path(&win_saved_games_base(), &self.company, &self.app, None)
        }
        #[cfg(target_os = "macos")]
        {
            bundle_path(&mac_app_support_base(), &self.bundle_id, Some("saves"))
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            app_path(&xdg_data_home_base(), &self.company, &self.app, Some("saves"))
        }
    }

    /// Logs & debug (NOT synced)
    /// - Windows: %LOCALAPPDATA%/company/app/logs/
    /// - macOS:   ~/Library/Logs/<bundle_id>/
    /// - Linux:   $XDG_STATE_HOME/company/app/logs/  (fallback ~/.local/state/logs)
    #[must_use] pub fn logs_dir(&self) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            app_path(
                &win_localappdata_base(),
                &self.company,
                &self.app,
                Some("logs"),
            )
        }
        #[cfg(target_os = "macos")]
        {
            bundle_path(&mac_logs_base(), &self.bundle_id, None)
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            app_path(
                &xdg_state_home_base(),
                &self.company,
                &self.app,
                Some("logs"),
            )
        }
    }

    /// Temp (short-lived; may be cleared on reboot)
    #[must_use] pub fn temp_dir(&self) -> PathBuf {
        env::temp_dir()
    }

    /* ---------------- Ensure & file helpers ---------------- */
    pub fn ensure_settings_dir(&self) -> Result<PathBuf> {
        ensure_dir(self.settings_dir())
    }
    pub fn ensure_saves_dir(&self) -> Result<PathBuf> {
        ensure_dir(self.saves_dir())
    }
    pub fn ensure_logs_dir(&self) -> Result<PathBuf> {
        ensure_dir(self.logs_dir())
    }
    pub fn ensure_parent_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if let Some(p) = path.as_ref().parent() {
            fs::create_dir_all(p)?;
        }
        Ok(())
    }

    #[must_use] pub fn settings_path(&self, name: &str) -> PathBuf {
        self.settings_dir().join(name)
    }
    #[must_use] pub fn saves_path(&self, name: &str) -> PathBuf {
        self.saves_dir().join(name)
    }
    #[must_use] pub fn logs_path(&self, name: &str) -> PathBuf {
        self.logs_dir().join(name)
    }
}

/* ================= joiners ================= */

fn ensure_dir<P: AsRef<Path>>(dir: P) -> Result<PathBuf> {
    fs::create_dir_all(&dir)?;
    Ok(dir.as_ref().to_path_buf())
}

/// Build ".../company/app[/suffix]"
#[allow(dead_code)]
fn app_path(base: &Path, company: &str, app: &str, suffix: Option<&str>) -> PathBuf {
    let mut p = base.to_path_buf();
    p.push(company);
    p.push(app);
    if let Some(s) = suffix {
        p.push(s);
    }
    p
}

/// Build ".../<bundle_id>[/suffix]"
fn bundle_path(base: &Path, bundle_id: &str, suffix: Option<&str>) -> PathBuf {
    let mut p = base.to_path_buf();
    p.push(bundle_id);
    if let Some(s) = suffix {
        p.push(s);
    }
    p
}

pub fn home() -> PathBuf {
    if cfg!(target_os = "windows") {
        env::var_os("USERPROFILE").or_else(|| env::var_os("HOME"))
    } else {
        env::var_os("HOME")
    }.map_or_else(|| PathBuf::from("/"), PathBuf::from)
}

/* ================= Platform bases ================= */

#[cfg(target_os = "windows")]
fn win_localappdata_base() -> PathBuf {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Users\Default\AppData\Local"))
}

#[cfg(target_os = "windows")]
fn win_saved_games_base() -> PathBuf {
    // Ideally use SHGetKnownFolderPath(FOLDERID_SavedGames).
    home().join("Saved Games")
}

#[cfg(target_os = "macos")]
fn mac_app_support_base() -> PathBuf {
    home().join("Library").join("Application Support")
}

#[cfg(target_os = "macos")]
fn mac_logs_base() -> PathBuf {
    home().join("Library").join("Logs")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn xdg_config_home_base() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn xdg_data_home_base() -> PathBuf {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".local").join("share"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn xdg_state_home_base() -> PathBuf {
    env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".local").join("state"))
}
