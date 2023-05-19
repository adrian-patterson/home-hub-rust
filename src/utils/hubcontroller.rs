use std::process::{Child, Command};

use anyhow::Error;

pub struct HubController {
    pub chrome_browser_process: Option<Child>,
    pub firefox_browser_process: Child,
    is_ha_dashboard_open: bool,
}

impl HubController {
    pub fn new() -> Self {
        let firefox_ha_process = Command::new("firefox")
            .arg("--start-maximized")
            .arg("--new-window")
            .arg("http://ha.local:8123")
            .spawn()
            .expect("Unable to create firefox browser process");
        Self {
            chrome_browser_process: None,
            firefox_browser_process: firefox_ha_process,
            is_ha_dashboard_open: true,
        }
    }

    pub fn prevent_screen_sleep() -> Result<(), Error> {
        Command::new("xset s 86400")
            .spawn()
            .expect("Unable to set display awake time");

        Command::new("xset dpms 86400 86400 86400")
            .spawn()
            .expect("Unable to set display dpms time");

        Ok(())
    }

    pub fn open_chrome_kiosk(&mut self, url: String) -> Result<(), Error> {
        self.close_chrome_kiosk()?;

        let new_browser_process = Command::new("chromium-browser")
            .arg("--kiosk")
            .arg("--new-window")
            .arg("--incognito")
            .arg(url)
            .spawn()
            .expect("Unable to create chrome kiosk process");

        self.chrome_browser_process = Some(new_browser_process);

        Ok(())
    }

    pub fn close_chrome_kiosk(&mut self) -> Result<(), Error> {
        match self.chrome_browser_process {
            Some(ref mut process) => {
                process.kill().expect("Unable to kill chrome kiosk process");
            }
            None => {}
        }

        Ok(())
    }

    pub fn open_firefox(&mut self, url: String) -> Result<(), Error> {
        self.firefox_browser_process.kill()?;

        let new_browser_process = Command::new("firefox")
            .arg("--start-maximized")
            .arg("--new-window")
            .arg(url)
            .spawn()
            .expect("Unable to create firefox browser process");

        self.firefox_browser_process = new_browser_process;

        self.is_ha_dashboard_open = false;

        Ok(())
    }

    pub fn close_firefox_reopen_ha(&mut self) -> Result<(), Error> {
        if !self.is_ha_dashboard_open {
            self.firefox_browser_process.kill()?;

            let ha_process = Command::new("firefox")
                .arg("--start-maximized")
                .arg("--new-window")
                .arg("http://ha.local:8123")
                .spawn()
                .expect("Unable to create firefox browser process");

            self.firefox_browser_process = ha_process;

            self.is_ha_dashboard_open = true;
        }

        Ok(())
    }

    pub fn wake_up_display() -> Result<(), Error> {
        Command::new("xset dpms force on")
            .spawn()
            .expect("Unable to wake up display");

        Ok(())
    }

    pub fn sleep_display() -> Result<(), Error> {
        Command::new("xset dpms force off")
            .spawn()
            .expect("Unable to wake up display");

        Ok(())
    }
}
