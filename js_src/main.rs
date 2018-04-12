#[macro_use]
extern crate stdweb;

extern crate failure;
extern crate serde;

#[macro_use]
extern crate failure_derive;

use failure::Error;

use stdweb::unstable::TryInto;
use stdweb::web::{document, window, EventTarget, HtmlElement, IEventTarget, IHtmlElement, INode,
                  IParentNode, Node};

use stdweb::web::event::{ClickEvent, IEvent};

macro_rules! page_err {
    ($error_variant:ident) => {
        || -> Error { PageError::$error_variant {}.into() }
    };
}

macro_rules! map_page_err {
    ($error_variant:ident) => {
        |_e| -> Error { PageError::$error_variant {}.into() }
    };
}

#[derive(Debug, Fail)]
enum PageError {
    #[fail(display = "no body on the page")]
    BodyError,
    #[fail(display = "no location object available")]
    NoLocationError,
    #[fail(display = "click taret data does not contain key `{}'", key)]
    MissingDataKey { key: &'static str },
    #[fail(display = "cannot create textarea node")]
    NoTextareaCreated,
}

fn select_fake(text: &str) -> Result<(), Error> {
    let d = document();
    let textarea = d.create_element("textarea")
        .map_err(map_page_err!(NoTextareaCreated))?;

    js! { @(no_return)
        // Prevent zooming on iOS
        @{&textarea}.style.fontSize = "12pt";
        // Reset box model
        @{&textarea}.style.border = "0";
        @{&textarea}.style.padding = "0";
        @{&textarea}.style.margin = "0";
    }
    js! { @(no_return)
        // Move element out of screen horizontally
        @{&textarea}.style.position = "absolute";
        @{&textarea}.style.left = "-9999px";
        // // Move element to the same position vertically
        // Move element to the same position vertically
        let yPosition = window.pageYOffset || document.documentElement.scrollTop;
        @{&textarea}.style.top = yPosition.toString() + "px";
        @{&textarea}.setAttribute("readonly", "");
    }

    textarea.set_text_content(text);

    d.query_selector("body")
        .unwrap() // selector syntax error
        .map(|body| {
            body.append_child(&textarea);
            js!{ @(no_return)
                @{textarea}.select();
                document.execCommand("copy");
                window.getSelection().removeAllRanges();
            };
            Ok(())
        })
        .ok_or_else(page_err!(BodyError))?
}

fn url_with_id(id: &str) -> Result<String, Error> {
    let location = window().location().ok_or_else(page_err!(NoLocationError))?;
    let protocol: String = js!(return @{&location}.protocol).try_into()?;
    let host: String = js!(return @{&location}.host).try_into()?;

    Ok(format!(
        "{protocol}//{host}/{id}",
        protocol = protocol,
        host = host,
        id = id
    ))
}

fn dataset_key(button: HtmlElement, key: &'static str) -> Result<String, Error> {
    let dataset = button.dataset();

    dataset
        .get(key)
        .ok_or_else(|| PageError::MissingDataKey { key }.into())
}

fn copy(e: ClickEvent) -> Result<(), Error> {
    let target: EventTarget = e.current_target().expect("no click target!");
    let button: HtmlElement = target.try_into()?;

    let id = dataset_key(button, "id")?;
    let url = url_with_id(&id)?;
    select_fake(&url)?;
    Ok(())
}

fn click(e: ClickEvent) {
    copy(e).unwrap_or_else(|err| console!(log, format!("{}", err)));
}

fn bind(button: Node) {
    button.add_event_listener(click);
}

fn do_all_stuff() {
    document()
        .query_selector_all(".copy")
        .unwrap() // selector syntax error
        .into_iter()
        .for_each(bind);
}

fn main() {
    stdweb::initialize();
    do_all_stuff();
}
