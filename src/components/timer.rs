use dioxus::prelude::*;
use std::time::Duration;
use crate::misc::{format_hms,format_ms};


// STRUCTS
// Timer logic and functions

#[derive(Clone, PartialEq,Copy)]
pub struct Timer {
    pub value: Signal<i32>,
    pub control: Signal<bool>,
}

impl Timer {
    /// Start an async loop that updates the timer.
    pub fn spawn(&mut self, step: i32) {
        let mut value = self.value.clone();
        let control = self.control.clone();

        use_future(move || async move {
            loop {
                if control() {
                    value.set(value() + step);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            
        });
    }

    /// Reset the timer to a specific value.
    pub fn reset(&mut self, new_value: i32) {
        self.value.set(new_value);
    }

    pub fn toggle(&mut self) {
        self.control.set(!(self.control)());
    }
    pub fn is_running(&self) -> bool {
        (self.control)()
    }
}


#[component]
pub fn Clock_modes(selected_val: Signal<String>) -> Element{
    rsx!{
        select{
            id:"selectbox",
            onchange:move |event| selected_val.set(event.value()),
            option {value: "opt1", "Timer"}
            option {value: "opt2", "Countdown"}
        }
    }
}

// UPTIMER

#[component]
pub fn Clockwise_timer(timer: Timer) -> Element {
    let ligh = false;
    rsx! {
        div {
            style: "display: flex; gap: 10px; align-items: center;",

            h1 { id: if ligh {"timer_box-light"} else {"timer_box"}, 
            "{format_hms((timer.value)())}" }
        }
            div { id: "buttons_div",
                Reset_timer { timer: timer.value },

                if timer.is_running() {
                        button { 
                            id: "button2",
                            onclick: move |_| timer.toggle(), 
                            "Stop Timer"
                        }
                } else {
                    Start_Timer { timer: timer }
                }
            
        }
    }
}


#[component]
pub fn Countdown_timer(timer: Timer, timer_break: Timer) -> Element {
    let time_end: Signal<bool> = use_signal(|| false);

    use_effect(move || {
        if (timer.value)() < 0 {
            timer.value.set(0);
            timer.toggle();
        }
    });

    rsx!(
        select {
            id:"selectbox",
            onchange: move |time_opt| timer.value.set(time_opt.value().parse().unwrap()),
            option {value: 60*20, "20 Min"},
            option {value: 60*10, "10 Min"},
            option {value: 30*1, "30 Segundos"},
        }

        div { //This div shows the main timer and breaktimer if break.control is true
            style: "display: flex; gap: 10px; align-items: center;",
            h1 { id: "timer_box", "{format_hms((timer.value)())}" }
            if (timer_break.control)() {
                h1 { id: "break_box", "{format_ms((timer_break.value)())}" }
            }
        }

        div {
            id: "buttons_div",
            Breaktimer{ timer: timer, timer_break: timer_break},
            Finishtimer { timer: timer }
            
            // Start
            if (timer_break.control)() {
                div {}
            } else if (timer.control)() {
                if (timer.value)() >= 0{
                    button {
                        id: "button2",
                        onclick: move |_| timer.toggle(),
                        "Pause"
                    }
                }
                else{
                    div{}
                }
            } else {
                Start_Timer { timer: timer } 
            }
        }
    )
}


#[component]
fn Breaktimer(timer: Timer, timer_break: Timer)-> Element{
    rsx!(
        if (timer_break.control)(){
           button {
           id:"button2",
           onclick: move |_| {timer.control.set(true);timer_break.control.set(false);}, 
           "End Break" 
           }   
        }
        else{
            button {
                id:"button2",
                onclick: move |_| {timer.control.set(false);timer_break.control.set(true);}, 
                "Start Break" //the idea is to popup like a timer aside the main timer, and play end sound
                }
            }
    )
}


#[component]
fn Finishtimer(timer: Timer)-> Element{
    rsx!(
        if (timer.control)(){
           button {
           id:"button2",
           onclick: move |_| {timer.control.set(true);timer.control.set(false);}, 
           "End Break" 
           }   
        }
        else{
            button {
                id:"button2",
                onclick: move |_| {timer.control.set(false);timer.control.set(true);}, 
                "Extend Timer"
                }
            }
    )
}
#[component]
pub fn Break_timer (mut main_control:Signal<bool>,break_control:Signal<bool>)-> Element{
    rsx!(
        if break_control(){
           button {
           id:"button2",
           onclick: move |_| {main_control.set(true);break_control.set(false);}, 
           "End Break" 
           }   
        }
        else{
            button {
                id:"button2",
                onclick: move |_| {main_control.set(false);break_control.set(true);}, 
                "Start Break" //the idea is to popup like a timer aside the main timer, and play end sound
                }
            }
    )
}



#[component]
fn Start_Timer(timer: Timer) -> Element {
    rsx! {
        button {
            id: "button2",
            onclick: move |_| timer.toggle(),
            "Start Timer"
        }
    }
}


#[component]
fn Reset_timer(timer:Signal<i32>) -> Element{

    rsx!{
        button { id: "button2",
            onclick: move |_| {
                timer.set(0);
            },
            "Reset timer"
        }
    }
}