use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};

use crate::app::components::events::*;

#[island]
pub fn Events() -> impl IntoView {
    view! {
        <main class="antialiased" id="events">
        <section id="eventlist" class="isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5">
            <p class="title mx-1 absolute top-3 left-3 w-content"><span class="mb-2 text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">Календар</span></p>
            <div class="-z-1 isolate absolute float-right inset-0 rounded-xl  [mask:linear-gradient(-45deg,black,transparent_70%)]">
                <div class="absolute -top-3 -translate-y-1/4 text-[440px] font-black leading-none bg-gradient-to-tr from-[#f26500] via-[#B56BFF] to-[#5bb6ff text-transparent bg-clip-text opacity-50 select-none"> @ </div>
                <div class="absolute -translate-y-1/2 -right-12 blur-2xl size-[200px] rounded-full bg-[#B56BFF] opacity-25"></div>
                <div class="absolute -translate-y-1/2 top-1/2 -right-12 blur-2xl size-[96px] rounded-full bg-[#f26500] opacity-25"></div>
                <div class="absolute -translate-y-1/2 top-full -right-12 blur-2xl size-[120px] rounded-full bg-[#5bb6ff] opacity-25"></div>
                <div class="z-1 absolute inset-1 ring-1 ring-inset ring-black/5 rounded-lg shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5 "></div>
            </div>

            <CalendarView />
            <Outlet/>
            // <EventList />
        </section>
        
        </main>
    }
}


// <main class="antialiased" id="events">
// <EventList />
// </main>