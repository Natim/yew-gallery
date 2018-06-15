extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

use failure::Error;
use yew::prelude::*;
use yew::format::{Nothing, Json};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub struct Model {
    fetching: bool,
    data: Option<String>,
    ft: Option<FetchTask>,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<String, Error>),
    Ignore,
}


/// This type is used to parse data from the Github API
#[derive(Deserialize, Debug)]
pub struct DataFromFile {
    tree: Vec<ImageFromFile>,
}

#[derive(Deserialize, Debug)]
pub struct ImageFromFile {
    url: String
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<FetchService> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            fetching: false,
            data: None,
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                let callback = env.send_back(move |response: Response<Json<Result<DataFromFile, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    println!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        Msg::FetchReady(data.map(|c| c.tree[0].url.clone()))
                    } else {
                        Msg::Ignore  // FIXME: Handle this error accordingly.
                    }
                });
                let request = Request::get("https://api.github.com/repos/Natim/elm-gallery/git/trees/master?recursive=1")
                    .header("Authorization", "c8b0db79c202ad52e0ba64a39f2e44fb4f41c543")
                    .body(Nothing)
                    .unwrap();
                let fetch_service: &mut FetchService = env.as_mut();
                let task = fetch_service.fetch(request, callback);
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<FetchService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::FetchData,>{ "Fetch Data" }</button>
                    { self.view_data() }
                </nav>
            </div>
        }
    }

}

impl Model {
    fn view_data<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<FetchService> + 'static,
    {
        if let Some(ref value) = self.data {
            html! {
                <p>{ format!("data:image/jpg;base64,{}", value) }</p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }
}
