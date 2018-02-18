#[macro_use]
extern crate stdweb;

extern crate failure;
extern crate serde;

#[macro_use]
extern crate failure_derive;
// #[macro_use]
// extern crate serde_derive;

use failure::Error;

use stdweb::unstable::TryInto;
use stdweb::web::{document, window, HtmlElement, IEventTarget, IHtmlElement,
                  INode, IParentNode};

use stdweb::web::event::{ClickEvent, IEvent};
// Shamelessly stolen from webplatform's TodoMVC example.
// macro_rules! enclose {
//     ( ($( $x:ident ),*) $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
// }

// use stdweb::private::UnimplementedException;
// use failure::{Backtrace, Fail};

// impl Fail for UnimplementedException {
//     fn cause(&self) -> Option<&Fail> {
//         self.inner.cause()
//     }
//
//     fn backtrace(&self) -> Option<&Backtrace> {
//         self.inner.backtrace()
//     }
// }

fn select_fake(text: &str) -> Result<(), Error> {
    let d = document();
    let textarea = d.create_element("textarea").unwrap(); // UnimplementedException

    js! {
        // Prevent zooming on iOS
        @{&textarea}.style.fontSize = "12pt";
        // Reset box model
        @{&textarea}.style.border = "0";
        @{&textarea}.style.padding = "0";
        @{&textarea}.style.margin = "0";
    }
    js! {
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

    let body_result = d.query_selector("body").unwrap(); // UnimplementedException

    match body_result {
        Some(body) => {
            body.append_child(&textarea);
            js!{
                @{textarea}.select();
                document.execCommand("copy");
                window.getSelection().removeAllRanges();
            };
        }
        None => return Err(PageError::BodyError {}.into()),
    }

    return Ok(());
}

fn url_with_id(id: &str) -> Result<String, Error> {
    let location = window().location().unwrap();
    let protocol: String = js!(return @{&location}.protocol).try_into()?;
    let host: String = js!(return @{&location}.host).try_into()?;

    let ret: String = format!(
        "{protocol}//{host}/{id}",
        protocol = protocol,
        host = host,
        id = id
    );

    Ok(ret)
}

#[derive(Debug, Fail)]
enum PageError {
    #[fail(display = "copy error")]
    CopyError,
    #[fail(display = "no body on the page")]
    BodyError,
}

fn copy(e: ClickEvent) -> Result<(), Error> {
    let button: HtmlElement = match e.current_target() {
        Some(node) => node.try_into().unwrap(),
        None => return Err(PageError::CopyError {}.into()),
    };

    let id: String = button.dataset().get("id").unwrap();
    let ret = url_with_id(&id).unwrap();
    select_fake(&ret)?;
    Ok(())
}

fn do_all_stuff() -> Result<(), Error> {
    let buttons = document().query_selector_all(".copy").unwrap();

    for btn in buttons {
        btn.add_event_listener(move |e: ClickEvent| match copy(e) {
            Ok(_) => {}
            Err(e) => {
                console!(log, format!("{:?}", e));
            }
        });
    }

    Ok(())
}

fn main() {
    stdweb::initialize();
    do_all_stuff().unwrap();
}
