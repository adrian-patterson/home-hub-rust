use std::process::{Child, Command};

use anyhow::{Error, Result};

const HOME_ASSISTANT_URL: &str = "http://ha.local:8123/";
const MAX_SLEEP_TIME_MS: &str = "98302";

pub struct HubController {
    pub chrome_browser_process: Option<Child>,
    pub ha_browser_process: Option<Child>,
}

impl HubController {
    pub fn new() -> Self {
        Self::prevent_screen_sleep().unwrap();

        let browser_process = Command::new("chromium-browser")
            .arg("--kiosk")
            .arg("--new-window")
            .arg(HOME_ASSISTANT_URL)
            .spawn()
            .expect("Unable to create chrome kiosk process");

        Self {
            chrome_browser_process: None,
            ha_browser_process: Some(browser_process),
        }
    }

    fn prevent_screen_sleep() -> Result<(), Error> {
        Command::new("xset")
            .arg("s")
            .arg(MAX_SLEEP_TIME_MS)
            .spawn()
            .expect("Unable to set display sleep time");

        Command::new("xset")
            .arg("dpms")
            .arg(MAX_SLEEP_TIME_MS)
            .arg(MAX_SLEEP_TIME_MS)
            .arg(MAX_SLEEP_TIME_MS)
            .spawn()
            .expect("Unable to set display sleep time");

        Ok(())
    }

    pub fn open_chrome_kiosk(&mut self, url: String) -> Result<(), Error> {
        self.close_all_kiosks()?;

        let new_browser_process = Command::new("chromium-browser")
            .arg("--kiosk")
            .arg("--new-window")
            .arg(url)
            .spawn()
            .expect("Unable to create chrome kiosk process");

        self.chrome_browser_process = Some(new_browser_process);

        Ok(())
    }

    pub fn close_all_kiosks(&mut self) -> Result<(), Error> {
        match self.chrome_browser_process {
            Some(ref mut process) => {
                process.kill().expect("Unable to kill chrome kiosk process");
                self.chrome_browser_process = None;
            }
            None => {}
        }

        match self.ha_browser_process {
            Some(ref mut process) => {
                process.kill().expect("Unable to kill chrome kiosk process");
                self.ha_browser_process = None;
            }
            None => {}
        }

        Ok(())
    }

    pub fn close_kiosk_and_open_ha_kiosk(&mut self) -> Result<(), Error> {
        match self.chrome_browser_process {
            Some(ref mut process) => {
                process.kill().expect("Unable to kill chrome kiosk process");
            }
            None => {}
        }

        match self.ha_browser_process {
            Some(_) => {}
            None => {
                let browser_process = Command::new("chromium-browser")
                    .arg("--kiosk")
                    .arg("--new-window")
                    .arg(HOME_ASSISTANT_URL)
                    .spawn()
                    .expect("Unable to create chrome kiosk process");

                self.ha_browser_process = Some(browser_process);
            }
        }

        Ok(())
    }

    pub fn wake_up_display(&mut self) -> Result<(), Error> {
        Command::new("xset")
            .arg("dpms")
            .arg("force")
            .arg("on")
            .spawn()
            .expect("Unable to wake up display");

        Self::prevent_screen_sleep()?;

        Ok(())
    }

    pub fn sleep_display(&mut self) -> Result<(), Error> {
        self.close_all_kiosks()?;

        Command::new("xset")
            .arg("dpms")
            .arg("force")
            .arg("off")
            .spawn()
            .expect("Unable to wake up display");

        Ok(())
    }
}
