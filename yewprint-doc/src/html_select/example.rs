use yew::prelude::*;
use yewprint::{HtmlSelect, Text};

pub struct Example {
    props: ExampleProps,
    link: &html::Scope<Self>,
    log_level: LogLevel,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
}

impl Component for Example {
    type Message = LogLevel;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            props,
            link,
            log_level: LogLevel::Info,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.log_level = msg;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div style="width: 400px; text-align: center;">
                <HtmlSelect<LogLevel>
                    options={vec![
                        (LogLevel::Trace, "TRACE".to_string()),
                        (LogLevel::Debug, "DEBUG".to_string()),
                        (LogLevel::Info, "INFO".to_string()),
                        (LogLevel::Warn, "WARN".to_string()),
                        (LogLevel::Error, "ERROR".to_string()),
                        (LogLevel::Off, "OFF".to_string()),
                    ]}
                    minimal={self.props.minimal}
                    fill={self.props.fill}
                    disabled={self.props.disabled}
                    large={self.props.large}
                    value={Some(self.log_level)}
                    onchange={self.link.callback(|x| x)}
                    title={format!("Selected: {:?}", self.log_level)}
                />
                <Text>{format!("Selected: {:?}", self.log_level)}</Text>
            </div>
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}
