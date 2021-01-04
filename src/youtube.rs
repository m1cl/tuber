// use web_view::*;
use std::future::Future;
use std::pin::Pin;
use yup_oauth2::authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate};
use yup_oauth2::{AccessToken, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    id: String,
    description: String,
    url: String,
    title: String,
    length: u8,
}
use std::process::Command;

type Playlists = Vec<PlaylistItem>;

#[derive(Default, Debug, Clone)]
pub struct Youtube {
    pub token: Option<AccessToken>,
    pub playlists: Option<Playlists>,
}
fn open_browser(url: &str) -> Result<(), ()> {
    Command::new("firefox").arg(url).output().unwrap();
    Ok(())
}
/// our custom delegate struct we will implement a flow delegate trait for:
/// in this case we will implement the `InstalledFlowDelegated` trait
async fn browser_user_url(url: &str, need_code: bool) -> Result<String, String> {
    if open_browser(url).is_ok() {
        println!("webbrowser was successfully opened.");
    }
    let def_delegate = DefaultInstalledFlowDelegate;
    def_delegate.present_user_url(url, need_code).await
}
#[derive(Copy, Clone)]
struct InstalledFlowBrowserDelegate;

/// here we implement only the present_user_url method with the added webbrowser opening
/// the other behaviour of the trait does not need to be changed.
impl InstalledFlowDelegate for InstalledFlowBrowserDelegate {
    /// the actual presenting of URL and browser opening happens in the function defined above here
    /// we only pin it
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(browser_user_url(url, need_code))
    }
}
impl Youtube {
    pub async fn auth() -> AccessToken {
        let secret = yup_oauth2::read_application_secret("clientsecret.json")
            .await
            .expect("clientsecret.json not found");

        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk("tokencache.json")
                // custom delegate to redirect user to the url
                .flow_delegate(Box::new(InstalledFlowBrowserDelegate))
                .build()
                .await
                .unwrap();

        // Permission
        let scopes = &["https://www.googleapis.com/auth/youtube.readonly"];

        // get the token and use to get data from youtube
        let token = match auth.token(scopes).await {
            Ok(token) => token,
            Err(err) => panic!("Problem: {:?}", err),
        };
        token
    }

    // fn get_playlists(&self) -> Playlists {
    //     self.playlists
    // }
    fn create_playlist_url(playlist_id: String) {
        let url = format!("https://youtube.com/:path+{}", playlist_id);
        println!("this should get the id from the playlistitem and create a link to that playlist, so we can use youtube-dl to download it");
    }
}
