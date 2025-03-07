use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

#[island]
pub fn Calendar() -> impl IntoView {
  // let (is_active, set_is_active) = signal(false);
  let is_active: RwSignal<bool> = RwSignal::new(false);
    let toggle_active = move |_| {
        is_active.set(!is_active.get())
    };
    
    view! {
        <section id="calendar" class="isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5" style="--grid-area:calendar">
        <p class="title mx-1 absolute top-3 left-3 w-content">
        <span class="mb-2 text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">Календар</span>
        </p>
        <div id="view">
            <span class="day">Mo</span>
            <span class="day">Tu</span>
            <span class="day">We</span>
            <span class="day">Th</span>
            <span class="day">Fr</span>
            <span class="day">Sa</span>
            <span class="day">Su</span>
            <button class="date faded">24</button>
            <button class="date faded">25</button>
            <button class="date faded">26</button>
            <button class="date faded">27</button>
            <button class="date faded">28</button>
            <button class="date">1</button>
            <button class="date">2</button>
            <button class="date">3</button>
            <button class="date">4</button>
            <button class="date current-day">5</button>
            <button class="date">6</button>
            <button class="date">7</button>
            <button class="date">8</button>
            <button class="date">9</button>
            <button class="date">10</button>
            <button class="date">11</button>
            <button class="date">12</button>
            <button class="date">13</button>
            <button class="date">14</button>
            <button class="date">15</button>
            <button class="date">16</button>
            <button class="date">17</button>
            <button class="date">18</button>
            <button class="date">19</button>
            <button class="date">20</button>
            <button class="date">21</button>
            <button class="date">22</button>
            <button class="date">23</button>
            <button class="date">24</button>
            <button class="date">25</button>
            <button class="date">27</button>
            <button class="date">27</button>
            <button class="date">28</button>
            <button class="date">29</button>
            <button class="date">30</button>
        </div>
        <div class="-z-1 isolate absolute float-right inset-0 rounded-xl  [mask:linear-gradient(-45deg,black,transparent_70%)]">
            <div class="absolute -top-3 -translate-y-1/4 text-[440px] font-black leading-none bg-gradient-to-tr from-[#f26500] via-[#B56BFF] to-[#5bb6ff text-transparent bg-clip-text opacity-50 select-none"> @ </div>
            <div class="absolute -translate-y-1/2 -right-12 blur-2xl size-[200px] rounded-full bg-[#B56BFF] opacity-25"></div>
            <div class="absolute -translate-y-1/2 top-1/2 -right-12 blur-2xl size-[96px] rounded-full bg-[#f26500] opacity-25"></div>
            <div class="absolute -translate-y-1/2 top-full -right-12 blur-2xl size-[120px] rounded-full bg-[#5bb6ff] opacity-25"></div>
            <div class="z-1 absolute inset-1 ring-1 ring-inset ring-black/5 rounded-lg shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5 "></div>
        </div>
        <div id="info"
        class=move || {
          // Conditionally toggle the expanded class
          if is_active.get() {
              "expanded"
          } else {
              ""
          }
      }>
          <div class="events">
            <div class="event">
              <div class="icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" stroke-width="2"/>
                  <path d="M13 2v7h7" stroke-width="2"/>
                </svg>
              </div>
              <div class="content">
                <div class="title">Event</div>
                <div class="subtitle">Desc</div>
              </div>
              <div class="date">5 August</div>
            </div>

            <div class="event">
              <div class="icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" stroke-width="2"/>
                  <circle cx="12" cy="10" r="3" stroke-width="2"/>
                </svg>
              </div>
              <div class="content">
                <div class="title">Event</div>
                <div class="subtitle">Desc</div>
              </div>
              <div class="date">2 August</div>
            </div>

            <div class="event">
              <div class="icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                  <path d="M8.5 14.5A2.5 2.5 0 0011 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 11-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 002.5 2.5z" stroke-width="2"/>
                </svg>
              </div>
              <div class="content">
                <div class="title">Event</div>
                <div class="subtitle">Desc</div>
              </div>
              <div class="date">28 July</div>
            </div>
          </div>

          <button class="show-hide-btn" on:click=toggle_active>
            <span class="show">Show All</span>
            <span class="hide">Hide</span>
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <path d="M4 6l4 4 4-4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </section>
    }
}
