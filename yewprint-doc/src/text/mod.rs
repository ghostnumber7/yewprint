mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};
use web_sys::HtmlInputElement;

pub struct TextDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TextDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        TextDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                ellipsize: false,
                text: String::from("Hello, world!"),
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Text"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <TextProps
                                callback={self.callback.clone()}
                                props={example_props.clone()}
                            />
                        })}
                    >
                        <Example ..example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    TextProps for ExampleProps =>
        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <Switch
                        onclick={self.update_props(|props, _| ExampleProps {
                            ellipsize: !props.ellipsize,
                            ..props
                        })}
                        checked={self.props.ellipsize}
                        label={html!("Ellipsize")}
                    />
                    <input
                        class="bp3-input"
                        oninput={self.update_props(|props, e: InputEvent| {
                            let input = e.target_dyn_into::<HtmlInputElement>();
                            let mut text = props.text;
                            input.map(|input| {
                                text = input.value();
                                if text.trim() == "" {
                                    text = "Hello, world!".to_string();
                                }
                            });
                            ExampleProps {
                                text,
                                ..props
                            }
                        })}
                        type="text"
                        value={self.props.text.clone()}
                    />
                </div>
            }
        }
}

crate::build_source_code_component!();
