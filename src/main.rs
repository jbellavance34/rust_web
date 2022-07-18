use serde::{Serialize,Deserialize};
use yew::prelude::*;
use reqwasm::http::Request;
use js_sys::JsString;
use web_sys::console;
use substring::Substring;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Choice {
    pub created_at: String,
    pub animal: String,
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Items {
    pub Items: Vec<Choice>
}

#[derive(PartialEq, Properties, Clone)]
struct ChoiceComponentsProps {
    pub choice: Choice
}
#[function_component(ChoiceComponent)]
fn choice_component(props: &ChoiceComponentsProps) -> Html {
    html! {
        <tbody >
            <tr >
                <td  class="table_td_th">{props.choice.name.to_owned()}</td>
                <td  class="table_td_th">{props.choice.animal.to_owned()}</td>
                <td  class="table_td_th">{props.choice.description.to_owned()}</td>
                <td  class="table_td_th">{props.choice.created_at.to_owned().substring(0,10)}</td>
            </tr>
        </tbody>
    }
}
#[function_component(App)]
fn app() -> Html {
    // fetch choices data
    let choices = use_state(|| None);
    let choices_clone = choices.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let endpoint = "https://cplxefze62zn5phxch2l4rnrhy0ncosx.lambda-url.ca-central-1.on.aws";
        let fetched_choices: Items = Request::get(&endpoint)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        choices.set(Some(fetched_choices));
    });
    html! {
        <div>
            <section class="container">
                <figure >
                    <img src="img/cat.jpg" alt="cute-cat" class="img"/> 
                    <figcaption>{"Cat of the day"}</figcaption>
                </figure>
                <figure >
                    <img src="img/dog.jpg" alt="cute-dog" class="img"/> 
                    <figcaption>{"Dog of the day"}</figcaption>
                </figure>
                <figure >
                    <img src="img/pet.jpg" alt="cute-pet" class="img"/> 
                    <figcaption>{"Cat of the day"}</figcaption>
                </figure>
            </section>
            <section class="liste">
                <div class="liste">
                    <h1>{"Liste des 10 derniers choix"}</h1>
                    <table class="table_td_th">
                        <thead>
                            <tr class="table_td_th">
                            <th class="table_td_th">{"Nom"}</th>
                            <th class="table_td_th">{"Animal"}</th>
                            <th class="table_td_th">{"Description"}</th>
                            <th class="table_td_th">{"Date"}</th>
                            <th class="table_td_th">{"Link"}</th>
                            </tr>
                        </thead>
                        {
                            match choices_clone.as_ref() {
                                Some(f) => {
                                    f.Items.iter().rev().take(10).map(|choice| {
                                        html! {
                                            <ChoiceComponent choice={choice.clone()}/>
                                        }
                                    }).collect()
                                
                                },
                                None => html!(
                                    { 
                                        html!  {
                                            <tbody >
                                                <tr >
                                                    <td  class="table_td_th">{"No data"}</td>
                                                    <td  class="table_td_th">{"No data"}</td>
                                                    <td  class="table_td_th">{"No data"}</td>
                                                    <td  class="table_td_th">{"No data"}</td>
                                                </tr>
                                            </tbody>
                                        }
                                    }
                                )

                            }
                        }
                    </table>
                    <h2>{"Sélectionnez votre animal préféré"}</h2>
                    <input
                        //onChange={event => setInput('name', event.target.value)}
                        class="selection"
                        //value={formState.name}
                        maxLength="50"
                        placeholder="Name"
                        required=true
                    />
                    <input
                        //onChange={event => setInput('description', event.target.value)}
                        class="selection"
                        maxLength="150"
                        //value={formState.description}
                        placeholder="Description"
                    />
                    <select
                        //onChange={event => setInput('animal', event.target.value)}
                        class="selection"
                        //value={formState.animal}
                        placeholder="Animal"
                    >
                        <option value="cat">{"Cat"}</option>
                        <option value="dog">{"Dog"}</option>
                        <option value="pet">{"Pet"}</option>
                    </select>
                </div>
            </section>   
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}