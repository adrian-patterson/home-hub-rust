use std::process::{Child, Command};

use anyhow::Error;

pub struct HubController {
    pub chrome_browser_process: Option<Child>,
}

impl HubController {
    pub fn new() -> Self {
        Command::new("firefox")
            .arg("--start-maximized")
            .arg("--new-window")
            .arg("http://ha.local:8123")
            .spawn()
            .expect("Unable to create firefox browser process");
        Self {
            chrome_browser_process: None,
        }
    }

    pub fn prevent_screen_sleep() -> Result<(), Error> {
        Command::new("xset")
            .arg("s")
            .arg("86400")
            .spawn()
            .expect("Unable to set display awake time");

        Command::new("xset")
            .arg("dpms")
            .arg("86400")
            .arg("86400")
            .arg("86400")
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
        Command::new("firefox")
            .arg(url)
            .spawn()
            .expect("Unable to create firefox browser process");

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