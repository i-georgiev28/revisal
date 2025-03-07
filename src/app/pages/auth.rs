use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};
use crate::auth::{self, *};

use leptos::web_sys::window;

#[island]
pub fn Authenticate() -> impl IntoView {
    let is_active: RwSignal<bool> = RwSignal::new(false);
    let toggle_active = move |_| {
        is_active.set(!is_active.get())
    };

    let (name, set_name) = signal("testuser".to_string());
    let (email, set_email) = signal("test@example.com".to_string());
    let (password, set_password) = signal("Test@Example123".to_string());


    let login: ServerAction<_> = ServerAction::<Login>::new();
    let signup: ServerAction<_> = ServerAction::<Signup>::new();

    // Effect::new(move |_| {
    //     if login.version().get() > 0 { // If the action has run at least once
    //         if let Some(res) = login.value().get() {
    //             if res.is_ok() {
    //                 if let Some(window) = leptos::web_sys::window() {
    //                     window.location().set_href("/app/dashboard").unwrap();
    //                 }
    //             }
    //         }
    //     }
    // });

    // Effect::new(move |_| {
    //     if signup.version().get() > 0 {
    //         if let Some(res) = signup.value().get() {
    //             if res.is_ok() {
    //                 if let Some(window) = leptos::web_sys::window() {
    //                     window.location().set_href("/app/dashboard").unwrap();
    //                 }
    //             }
    //         }
    //     }
    // });

    view! {
        <main id="auth"         class=move || {
            if is_active.get() {
                "active "
            } else {
                ""
            }
        }>
        <div
        class="absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg [mask:linear-gradient(10deg,black,transparent_66%)] ">
        
        </div>
        <div class="-z-10 absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg bg-gradient-to-tr from-[#f26500] via-[#B56BFF] via-66% to-[#5bb6ff] [mask:linear-gradient(-45deg,black,transparent_95%)] opacity-10"></div>
        <div class="absolute -top-12 -right-12 size-[200px] rounded-full bg-gradient-to-br from-[#f26500] via-[#B56BFF] via-35% to-[#5bb6ff] [mask:radial-gradient(circle_at_75%_25%,transparent_60%,black)] opacity-15"></div>   
        <div class="absolute -top-16 -left-12 -rotate-45 size-[200px] rounded-full bg-gradient-to-br from-[#f26500] via-[#B56BFF] via-35% to-[#5bb6ff] [mask:radial-gradient(circle_at_75%_25%,transparent_60%,black)] opacity-15"></div> 
        <div class="absolute -bottom-12 -right-4 translate-x-1/2 rotate-45 size-[200px] rounded-full bg-gradient-to-br from-[#f26500] via-[#B56BFF] via-35% to-[#5bb6ff] [mask:radial-gradient(circle_at_75%_25%,transparent_60%,black)] opacity-15"></div> 
    <div class="form-box login">
        <form on:submit=move |ev| {
            ev.prevent_default();
            login.dispatch(auth::Login { email: email(), password: password(), remember: Some("Yes".to_string()) });

        }>
        <h1
        class="font-medium text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        Влез</h1>
            <div class="input-box">
                <input bind:value=(email, set_email) type="text" placeholder="Имейл" required  class="ring-1 ring-inset ring-black/10 rounded-lg"/>
                <i class="bx bxs-user"></i>
            </div>
            <div class="input-box">
                <input bind:value=(password, set_password) type="password" placeholder="Парола" required  class="ring-1 ring-inset ring-black/10 rounded-lg"/>
                <i class="bx bxs-lock-alt" ></i>
            </div>
            <div class="forgot-link">
                <a href="#">{"Забравена парола?"}</a>
            </div>
            <button type="submit" class="btn">{"Влез"}</button>
        </form>
    </div>

    <div class="form-box register">
    <form on:submit=move |ev| {
        ev.prevent_default();
        signup.dispatch(auth::Signup {username: name(), email: email(), password: password(), password_confirmation: password(), remember: Some("Yes".to_string()) });   
    }>
        <h1
        class="font-medium text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        Създай акаунт</h1>
            <div class="input-box">
                <input bind:value=(name, set_name) type="text" placeholder="Потребителско име" class="ring-1 ring-inset ring-black/10 rounded-lg" required />
                <i class="bx bxs-user"></i>
            </div>
            <div class="input-box">
                <input bind:value=(email, set_email) type="email" placeholder="Имейл" required class="ring-1 ring-inset ring-black/10 rounded-lg" />
                <i class="bx bxs-envelope" ></i>
            </div>
            <div class="input-box">
                <input bind:value=(password, set_password) type="password" placeholder="Парола" required class="ring-1 ring-inset ring-black/10 rounded-lg" />
                <i class="bx bxs-lock-alt" ></i>
            </div>
            <button type="submit" class="btn">{"Регистрирай се"}</button>
        </form>
    </div>

    <div class="toggle-box">
        <div class="toggle-panel toggle-left">
            <h1>{"Добре дошъл!"}</h1>
            <p >{"Нямаш акаунт?"}</p>
            <button on:click=toggle_active class="btn register-btn">{"Регистрирай се"}</button>
        </div>

        <div class="toggle-panel toggle-right">
            <h1>{"Добре дошъл!"}</h1>
            <p>{"Вече имаш акаунт?"}</p>
            <button on:click=toggle_active class="btn login-btn">{"Влез"}</button>
        </div>
    </div>
</main>
    }    
}