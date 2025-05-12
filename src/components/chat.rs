use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let current_user = user.username.borrow().clone();

        html! {
            <div class="flex w-screen h-screen bg-gray-100">
                // Sidebar with users
                <div class="flex-none w-72 h-screen bg-white shadow-lg overflow-hidden flex flex-col">
                    <div class="bg-indigo-600 text-white py-4 px-5 flex items-center">
                        <svg class="h-7 w-7 text-white mr-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                        </svg>
                        <h1 class="text-xl font-bold">{"Online Users"}</h1>
                    </div>
                    <div class="flex-grow overflow-y-auto">
                        {
                            if self.users.is_empty() {
                                html! {
                                    <div class="flex items-center justify-center h-full text-gray-500">
                                        {"No users online"}
                                    </div>
                                }
                            } else {
                                self.users.clone().iter().map(|u| {
                                    html!{
                                        <div class="flex items-center p-4 border-b border-gray-100 hover:bg-gray-50 transition duration-150">
                                            <div class="relative">
                                                <img class="w-12 h-12 rounded-full object-cover" src={u.avatar.clone()} alt="avatar"/>
                                                <span class="absolute bottom-0 right-0 w-3 h-3 bg-green-400 border-2 border-white rounded-full"></span>
                                            </div>
                                            <div class="ml-4">
                                                <div class="font-medium">
                                                    {u.name.clone()}
                                                </div>
                                                <div class="text-xs text-gray-500">
                                                    {"Online"}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                </div>
                
                // Main chat area
                <div class="grow h-screen flex flex-col">
                    // Chat header
                    <div class="flex items-center h-16 bg-white shadow-sm z-10 px-6">
                        <div class="mr-4">
                            <svg class="w-6 h-6 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                            </svg>
                        </div>
                        <div>
                            <h2 class="text-lg font-semibold">{"YewChat"}</h2>
                            <div class="text-xs text-gray-500">{format!("Logged in as {}", current_user)}</div>
                        </div>
                    </div>
                    
                    // Messages area
                    <div class="w-full flex-grow overflow-auto bg-gray-50 p-4">
                        {
                            if self.messages.is_empty() {
                                html! {
                                    <div class="flex flex-col items-center justify-center h-full text-gray-500">
                                        <svg class="w-16 h-16 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                        </svg>
                                        <p class="mt-2">{"No messages yet. Start the conversation!"}</p>
                                    </div>
                                }
                            } else {
                                self.messages.iter().map(|m| {
                                let is_own_message = m.from == current_user;
                                
                                let user = self.users.iter()
                                    .find(|u| u.name == m.from)
                                    .cloned()
                                    .unwrap_or_else(|| UserProfile {
                                        name: m.from.clone(), 
                                        avatar: format!("https://avatars.dicebear.com/api/adventurer-neutral/{}.svg", m.from)
                                    });
                                
                                html!{
                                    <div class={classes!(
                                        "flex",
                                        "mb-4",
                                        "max-w-xl",
                                        if is_own_message { "ml-auto" } else { "" }
                                    )}>
                                        if !is_own_message {
                                            <img class="w-8 h-8 rounded-full mr-2 self-end" src={user.avatar.clone()} alt="avatar"/>
                                        }
                                            <div class={classes!(
                                                "p-3",
                                                "rounded-lg",
                                                "shadow-sm",
                                                if is_own_message { "bg-indigo-600 text-white" } else { "bg-white" },
                                                if is_own_message { "rounded-br-none" } else { "rounded-bl-none" }
                                            )}>
                                                if !is_own_message {
                                                    <div class="text-xs font-medium text-gray-500">
                                                        {&m.from}
                                                    </div>
                                                }
                                                <div class={classes!(
                                                    if is_own_message { "text-white" } else { "text-gray-800" }
                                                )}>
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-1 max-w-xs rounded" src={m.message.clone()}/>
                                                    } else {
                                                        {&m.message}
                                                    }
                                                </div>
                                            </div>
                                            if is_own_message {
                                                <img class="w-8 h-8 rounded-full ml-2 self-end" src={user.avatar.clone()} alt="avatar"/>
                                            }
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                    
                    // Message input
                    <div class="w-full bg-white p-4 shadow-lg">
                        <div class="flex items-center rounded-full bg-gray-100 overflow-hidden px-4 py-1">
                            <input 
                                ref={self.chat_input.clone()} 
                                type="text" 
                                placeholder="Type a message..." 
                                class="block w-full py-2 pl-2 bg-transparent outline-none"
                                name="message" 
                                required=true 
                                onkeypress={ctx.link().batch_callback(|e: KeyboardEvent| {
                                    if e.key() == "Enter" {
                                        Some(Msg::SubmitMessage)
                                    } else {
                                        None
                                    }
                                })}
                            />
                            <button 
                                onclick={submit} 
                                class="p-2 ml-2 bg-indigo-600 rounded-full flex justify-center items-center hover:bg-indigo-700 transition duration-150"
                            >
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 fill-white">
                                    <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
