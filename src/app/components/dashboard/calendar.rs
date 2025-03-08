use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};
use std::cmp::{max, min};
use crate::model::{self, *};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};

#[island]
pub fn Calendar() -> impl IntoView {
  // let (is_active, set_is_active) = signal(false);
  let is_active: RwSignal<bool> = RwSignal::new(false);
    let toggle_active = move |_| {
        is_active.set(!is_active.get())
    };

    let values = (1..=31).collect::<Vec<_>>();
    let current_day = Local::now().day();
    let today = move || {
      let day_val = clamp(1, current_day, 31);
      Local::now().date_naive().with_day(day_val).unwrap() // Get today's date (YYYY-MM-DD)
    };

    let eventlist =
    Resource::new(move || (), |_| get_events());

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
            {values.iter().map(|&day| {
              let class = if day == current_day { "date current-day" } else if day < current_day { "faded" } else { "date" };
              view! { <button class={class}>{day}</button> }
          }).collect_view()}
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
          <Suspense
          fallback=move || view! { <p class="text-gray-500">Loading events.</p> }
          >
          {
            move || match eventlist.get() {
                Some(Ok(events)) => {
                    // Filter events for the current day
                    let todays_events: Vec<EventRecord> = events.into_iter().filter(|event| {
                        NaiveDateTime::from_timestamp_opt(event.timestamp, 0)
                            .map(|dt| dt.date()) == Some(today())
                    }).collect();
        
                    if !todays_events.is_empty() {
                        Either::Left(
                            todays_events.into_iter().map(|event| view! {
                                  <div class="event">
                                  <div class="icon">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                      <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" stroke-width="2"/>
                                      <path d="M13 2v7h7" stroke-width="2"/>
                                    </svg>
                                  </div>
                                  <div class="content">
                                    <div class="title">{event.name}</div>
                                    <div class="subtitle">{event.description}</div>
                                  </div>
                                  <div class="date">
                                            {NaiveDateTime::from_timestamp(event.timestamp, 0)
                                            .date()
                                            .format("%d %B")
                                            .to_string()}
                                  </div>
                                  </div>
                                // <div class="event" tabindex="0">
                                //     <div class="icon">
                                //         <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                //             <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" stroke-width="2"/>
                                //             <path d="M13 2v7h7" stroke-width="2"/>
                                //         </svg>
                                //         <button on:click=move |_| {
                                //           unschedule_event.dispatch(model::event::DeleteEvent { event_id: event.id.clone() });
                                //         }
                                //         type="button"
                                //         class="w-full h-full outline-none bg-red-400 text-white">X</button>
                                //     </div>
                                //     <div class="content">
                                //         <div class="title">{event.name}</div>
                                //         <div class="subtitle">{event.description}</div>
                                //     </div>
                                //     <div class="date">
                                //         {NaiveDateTime::from_timestamp(event.timestamp, 0)
                                //             .date()
                                //             .format("%d %B")
                                //             .to_string()}
                                //     </div>
                                // </div>
                            }).collect::<Vec<_>>()
                        )
                    } else {
                        Either::Right(view! { <p class="text-gray-500">No events for {format_with_ordinal(today())}</p> })
                    }
                }
                _ => Either::Right(view! { <p class="text-gray-500">No events for {format_with_ordinal(today())}</p> } ),
            }
          }
          </Suspense>
            // <div class="event">
            //   <div class="icon">
            //     <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            //       <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" stroke-width="2"/>
            //       <path d="M13 2v7h7" stroke-width="2"/>
            //     </svg>
            //   </div>
            //   <div class="content">
            //     <div class="title">Event</div>
            //     <div class="subtitle">Desc</div>
            //   </div>
            //   <div class="date">5 August</div>
            // </div>

            // <div class="event">
            //   <div class="icon">
            //     <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            //       <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" stroke-width="2"/>
            //       <circle cx="12" cy="10" r="3" stroke-width="2"/>
            //     </svg>
            //   </div>
            //   <div class="content">
            //     <div class="title">Event</div>
            //     <div class="subtitle">Desc</div>
            //   </div>
            //   <div class="date">2 August</div>
            // </div>

            // <div class="event">
            //   <div class="icon">
            //     <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
            //       <path d="M8.5 14.5A2.5 2.5 0 0011 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 11-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 002.5 2.5z" stroke-width="2"/>
            //     </svg>
            //   </div>
            //   <div class="content">
            //     <div class="title">Event</div>
            //     <div class="subtitle">Desc</div>
            //   </div>
            //   <div class="date">28 July</div>
            // </div>
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



fn clamp(value: u32, min_value: u32, max_value: u32) -> u32 {
  max(min_value, min(value, max_value))
}

fn format_with_ordinal(date: NaiveDate) -> String {
  let day = date.day();
  let suffix = match day {
      1 | 21 | 31 => "st",
      2 | 22 => "nd",
      3 | 23 => "rd",
      _ => "th",
  };
  let month = date.format("%B").to_string(); // Get full month name
  format!("{}{} {}", day, suffix, month)
}
