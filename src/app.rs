use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::dialog::DialogService;
use web_sys::{Document, Location, window, MediaDevices};

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    hide_chat_mobile_view: bool,
    enable_mic: bool,
    enable_camera: bool,
}

pub enum Msg {
    OpenChat,
    HideChat,
    ToggleMic,
    ToggleCamera,
    InviteUser,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            hide_chat_mobile_view: true,
            enable_mic: true,
            enable_camera: true,
        };
        
        App {
            link,
            state,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenChat => {
                self.state.hide_chat_mobile_view = false;
            }
            Msg::HideChat => {
                self.state.hide_chat_mobile_view = true;
            }
            Msg::ToggleMic => {
                self.state.enable_mic = !self.state.enable_mic;
            }
            Msg::ToggleCamera => {
                self.state.enable_camera = !self.state.enable_camera;
            }
            Msg::InviteUser => {
                let href = location().href().expect("href unavailable");
                
                let message = "Copy this link and send it to people you want to meet with:";
                DialogService::prompt(&message, Some(&href));
            }
        }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        let classes_main_left = if self.state.hide_chat_mobile_view {
            "main__left"
        } else {
            "main__left disable"
        };
        let classes_main_right = if self.state.hide_chat_mobile_view {
            "main__right"
        } else {
            "main__right disable"
        };
        let classes_header_back = if self.state.hide_chat_mobile_view {
            "header__back"
        } else {
            "header__back disable"
        };
        let classes_mic = if self.state.enable_mic {
            "fa fa-microphone"
        } else {
            "fa fa-microphone-slash background__red"
        };
        let classes_button_mic = if self.state.enable_mic {
            "options__button"
        } else {
            "options__button background__red"
        };
        let classes_camera = if self.state.enable_camera {
            "fa fa-video-camera"
        } else {
            "fa fa-video-slash background__red"
        };
        let classes_button_camera = if self.state.enable_camera {
            "options__button"
        } else {
            "options__button background__red"
        };

        html! {
          <div>
            <div class="header">
              <div class="logo">
                <div class=classes_header_back onclick=self.link.callback(|_| {Msg::HideChat})>
                  <i class="fas fa-angle-left" />
                </div>
                <h2>{"Video Chat"}</h2>
              </div>
            </div>
            <div class="main">
            <div class=classes_main_left>
              <div class="videos__group">
                <div id="video-grid" />
              </div>
              <div class="options">
                <div class="options__left">
                  <div id="stopVideo" class=classes_button_camera onclick=self.link.callback(|_| {Msg::ToggleCamera})>
                    <i class=classes_camera />
                  </div>
                  <div id="muteButton" class=classes_button_mic onclick=self.link.callback(|_| {Msg::ToggleMic})>
                    <i class=classes_mic />
                  </div>
                  <div id="showChat" class="options__button" onclick=self.link.callback(|_| Msg::OpenChat)>
                    <i class="fa fa-comment" />
                  </div>
                </div>
                <div class="options__right">
                  <div id="inviteButton" class="options__button" onclick=self.link.callback(|_| {Msg::InviteUser})>
                    <i class="fas fa-user-plus" />
                  </div>
                </div>
              </div>
            </div>
            <div class=classes_main_right>
              <div class="main__chat_window">
                  <div class="messages" />
              </div>
              <div class="main__message_container">
                <input id="chat_message" type="text" autocomplete="off" placeholder="Type message here..." />
                <div id="send" class="options__button">
                  <i class="fa fa-plus" aria-hidden="true" />
                </div>
              </div>
            </div>
          </div>
          </div>
        }
    }
}

fn document() -> Document {
    window().unwrap().document().expect("Document unavailable")
}

fn location() -> Location {
    document().location().expect("Location unavailable")
}
