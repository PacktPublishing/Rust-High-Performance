#![feature(proc_macro)]

extern crate maud;
use maud::html;

fn main() {
    use maud::PreEscaped;

    let user_name = "FooBar";
    let markup = html! {
        (PreEscaped("<!DOCTYPE html>"))
        html {
            head {
                title { "Test website" }
                meta charset="UTF-8";
            }
            body {
                header {
                    nav {
                        ul {
                            li { "Home" }
                            li { "Contact Us" }
                        }
                    }
                }
                main {
                    h1 { "Welcome to our test template!" }
                    p { "Hello, " (user_name) "!" }
                }
                footer {
                    p { "Copyright © 2017 - someone" }
                }
            }
        }
    };
    println!("{}", markup.into_string());
}
