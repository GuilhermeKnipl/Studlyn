use dioxus::prelude::*;
use rusqlite::Connection;
use studlyn::components::timer::*;
//use dotenv::dotenv;
use studlyn::sql::builder;


fn main() {
    dioxus::launch(app);
}


static CSS: Asset = asset!("/assets/main.css");
static CSS_MISC: Asset = asset!("/assets/misc.css");

#[component]
fn app() -> Element {
    //dotenv().ok();
    let a = builder::get_path();
    let conn: Connection = builder::establish_connection();    


    let timer = Timer { value: use_signal(|| 0), control: use_signal(|| false) };
    let countdown = Timer { value: use_signal(|| 20 * 60), control: use_signal(|| false) };
    let break_timer = Timer { value: use_signal(|| 5 * 60), control: use_signal(|| false) };

    timer.clone().spawn(1);
    countdown.clone().spawn(-1);
    break_timer.clone().spawn(-1);


    let select_value = use_signal(|| "opt1".to_string());

    rsx! {
        document::Stylesheet { href: CSS }
        document::Stylesheet { href: CSS_MISC }


        div { style: "height: 20px;" }

        Clock_modes { selected_val: select_value}
        match select_value().as_str() {
            "opt1" => rsx! { Clockwise_timer { timer: timer} },
            "opt2" => rsx! { Countdown_timer { timer:countdown,timer_break: break_timer } },
            _ => rsx! { h1 { "none" } }
        }
        if (countdown.value)() <= 60*19 {
            h1 { "end" }
        }
        
    }
}




