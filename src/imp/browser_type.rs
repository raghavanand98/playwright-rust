use crate::imp::{
    browser::Browser,
    browser_context::BrowserContext,
    core::*,
    prelude::*,
    utils::{BrowserChannel, ColorScheme, Geolocation, HttpCredentials, ProxySettings, Viewport}
};

#[derive(Debug)]
pub(crate) struct BrowserType {
    channel: ChannelOwner,
    name: String,
    executable: PathBuf
}

impl BrowserType {
    pub(crate) fn try_new(channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { name, executable } = serde_json::from_value(channel.initializer.clone())?;
        Ok(Self {
            channel,
            name,
            executable
        })
    }

    pub(crate) fn name(&self) -> &str { &self.name }

    pub(crate) fn executable(&self) -> &Path { &self.executable }

    pub(crate) async fn launch(
        &self,
        args: LaunchArgs<'_, '_, '_>
    ) -> Result<Weak<Browser>, Arc<Error>> {
        let res = send_message!(self, "launch", args);
        let guid = only_guid(&res)?;
        let b = get_object!(self.context()?.lock().unwrap(), &guid, Browser)?;
        Ok(b)
    }

    pub(crate) async fn launch_persistent_context(
        &self,
        args: LaunchPersistentContextArgs<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>
    ) -> Result<Weak<BrowserContext>, Arc<Error>> {
        let res = send_message!(self, "launchPersistentContext", args);
        let guid = only_guid(&res)?;
        let b = get_object!(self.context()?.lock().unwrap(), &guid, BrowserContext)?;
        Ok(b)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaunchArgs<'a, 'b, 'c> {
    #[serde(rename = "executablePath")]
    pub(crate) executable: Option<&'a Path>,
    pub(crate) args: Option<&'b [String]>,
    pub(crate) ignore_all_default_args: Option<bool>,
    #[serde(rename = "handleSIGINT")]
    pub(crate) handle_sigint: Option<bool>,
    #[serde(rename = "handleSIGTERM")]
    pub(crate) handle_sigterm: Option<bool>,
    #[serde(rename = "handleSIGHUP")]
    pub(crate) handle_sighup: Option<bool>,
    pub(crate) timeout: Option<f64>,
    pub(crate) env: Option<Map<String, Value>>,
    pub(crate) headless: Option<bool>,
    pub(crate) devtools: Option<bool>,
    pub(crate) proxy: Option<ProxySettings>,
    #[serde(rename = "downloadsPath")]
    pub(crate) downloads: Option<&'c Path>,
    #[serde(rename = "slowMo")]
    pub(crate) slowmo: Option<f64>,
    pub(crate) chromium_sandbox: Option<f64>,
    pub(crate) firefox_user_prefs: Option<Map<String, Value>>,
    pub(crate) channel: Option<BrowserChannel>
}

impl RemoteObject for BrowserType {
    fn channel(&self) -> &ChannelOwner { &self.channel }
    fn channel_mut(&mut self) -> &mut ChannelOwner { &mut self.channel }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    name: String,
    #[serde(rename = "executablePath")]
    executable: PathBuf
}

// launch args | context args | {user_data_dir: }
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LaunchPersistentContextArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    user_data_dir: &'a Path,
    sdk_language: &'static str,

    #[serde(rename = "executablePath")]
    pub(crate) executable: Option<&'b Path>,
    pub(crate) args: Option<&'c [String]>,
    pub(crate) ignore_all_default_args: Option<bool>,
    #[serde(rename = "handleSIGINT")]
    pub(crate) handle_sigint: Option<bool>,
    #[serde(rename = "handleSIGTERM")]
    pub(crate) handle_sigterm: Option<bool>,
    #[serde(rename = "handleSIGHUP")]
    pub(crate) handle_sighup: Option<bool>,
    pub(crate) timeout: Option<f64>,
    pub(crate) env: Option<Map<String, Value>>,
    pub(crate) headless: Option<bool>,
    pub(crate) devtools: Option<bool>,
    pub(crate) proxy: Option<ProxySettings>,
    #[serde(rename = "downloadsPath")]
    pub(crate) downloads: Option<&'d Path>,
    #[serde(rename = "slowMo")]
    pub(crate) slowmo: Option<f64>,

    pub(crate) viewport: Option<Viewport>,
    pub(crate) screen: Option<Viewport>,
    pub(crate) no_default_viewport: Option<bool>,
    #[serde(rename = "ignoreHTTPSErrors")]
    pub(crate) ignore_https_errors: Option<bool>,
    #[serde(rename = "javaScriptEnabled")]
    pub(crate) js_enabled: Option<bool>,
    #[serde(rename = "bypassCSP")]
    pub(crate) bypass_csp: Option<bool>,
    pub(crate) user_agent: Option<&'e str>,
    pub(crate) locale: Option<&'f str>,
    pub(crate) timezone_id: Option<&'g str>,
    pub(crate) geolocation: Option<Geolocation>,
    pub(crate) permissions: Option<&'h [String]>,
    #[serde(rename = "extraHTTPHeaders")]
    pub(crate) extra_http_headers: Option<HashMap<String, String>>,
    pub(crate) offline: Option<bool>,
    pub(crate) http_credentials: Option<&'i HttpCredentials>,
    pub(crate) device_scale_factor: Option<f64>,
    pub(crate) is_mobile: Option<bool>,
    pub(crate) has_touch: Option<bool>,
    pub(crate) color_scheme: Option<ColorScheme>,
    pub(crate) accept_downloads: Option<bool>,
    pub(crate) chromium_sandbox: Option<bool>,
    pub(crate) record_video: Option<RecordVideo<'j>>,
    pub(crate) record_har: Option<RecordHar<'k>>,
    pub(crate) channel: Option<BrowserChannel>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordVideo<'a> {
    pub dir: &'a Path,
    pub size: Option<Viewport>
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordHar<'a> {
    pub path: &'a Path,
    pub omit_content: Option<bool>
}

impl<'a> LaunchPersistentContextArgs<'a, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub(crate) fn new(user_data_dir: &'a Path) -> Self {
        let sdk_language = "rust";
        Self {
            user_data_dir,
            sdk_language,
            executable: None,
            args: None,
            ignore_all_default_args: None,
            handle_sigint: None,
            handle_sigterm: None,
            handle_sighup: None,
            timeout: None,
            env: None,
            headless: None,
            devtools: None,
            proxy: None,
            downloads: None,
            slowmo: None,
            viewport: None,
            screen: None,
            no_default_viewport: None,
            ignore_https_errors: None,
            js_enabled: None,
            bypass_csp: None,
            user_agent: None,
            locale: None,
            timezone_id: None,
            geolocation: None,
            permissions: None,
            extra_http_headers: None,
            offline: None,
            http_credentials: None,
            device_scale_factor: None,
            is_mobile: None,
            has_touch: None,
            color_scheme: None,
            accept_downloads: None,
            chromium_sandbox: None,
            record_video: None,
            record_har: None,
            channel: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::imp::playwright::Playwright;

    crate::runtime_test!(launch, {
        let driver = Driver::install().unwrap();
        let conn = Connection::run(&driver.executable()).unwrap();
        let p = Playwright::wait_initial_object(&conn).await.unwrap();
        let p = p.upgrade().unwrap();
        let chromium = p.chromium().upgrade().unwrap();
        let res = chromium.launch(LaunchArgs::default()).await;
        dbg!(&res);
        res.unwrap();
    });

    // crate::runtime_test!(launch_persistent_context, {
    //    let driver = Driver::install().unwrap();
    //    let conn = Connection::run(&driver.executable()).unwrap();
    //    let p = Playwright::wait_initial_object(&conn).await.unwrap();
    //    let p = p.upgrade().unwrap();
    //    let firefox = p.firefox().upgrade().unwrap();
    //    let res = firefox
    //        .launch_persistent_context(LaunchPersistentContextArgs::new(".".as_ref()))
    //        .await;
    //    dbg!(&res);
    //    res.unwrap();
    //});
}
