use leptos::{portal::Portal, prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *, hooks::*, params::Params};
use std::cmp::{max, min};
use crate::model::{self, *};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};

#[derive(Params, PartialEq)]
struct DayParams {
    day: Option<u32>,
}

#[island]
pub fn EventList() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (description, set_description) = signal("".to_string());
    let (label, set_label) = signal("".to_string());
    let (hour, set_hour) = signal("12:00".to_string());

    let params = use_params::<DayParams>();  
    // day: || -> u32
    let day = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.day)
            .unwrap_or_default()
    };

    let new_event = ServerAction::<CreateEvent>::new();
    let unschedule_event = ServerAction::<DeleteEvent>::new();

    let eventlist =
        Resource::new(move || (new_event.version().get(), unschedule_event.version().get()), |_| get_events());
    
    let is_modal_open: RwSignal<bool> = RwSignal::new(false);
    let toggle_modal = move |_| {
        is_modal_open.set(!is_modal_open.get())
    };

    // Instead of calculating `today` once, track changes reactively
    let today = move || {
        let day_val = clamp(1, day(), 31);
        Local::now().date_naive().with_day(day_val).unwrap() // Get today's date (YYYY-MM-DD)
    };
  
    view! {
        <div id="info" class="expanded">
          <div class="events">
            <button
              on:click=toggle_modal
              class="self-end justify-self-end mx-2 z-[100] bg-white/80 cursor-pointer hover:bg-white/60 isolate max-w-4 flex p-6 text-lg max-h-4 ring-1 ring-inset ring-black/10 rounded-lg text-center items-center justify-center">
              <span class="leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">+</span>
            </button>
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
                                  <div class="event" tabindex="0">
                                      <div class="icon">
                                          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                              <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" stroke-width="2"/>
                                              <path d="M13 2v7h7" stroke-width="2"/>
                                          </svg>
                                          <button class="w-full h-full outline-none bg-red-400 text-white">X</button>
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
          </div>
        </div>

        <Portal>
            <div id="modal" class=move || {
                    if is_modal_open.get() {
                        "open transition-all duration-200"
                    } else {
                        "transition-all duration-200"
                    }
                }>
                <div class="relative px-4 min-h-screen flex items-center justify-center">
                <button on:click=toggle_modal class="overlay"></button>
                <div class="bg-white rounded-lg max-w-md mx-auto p-4 fixed inset-x-0 bottom-0 z-50 mb-4 mx-4 relative">
                  <div class="md:flex items-center">
                    <div class="rounded-full border border-gray-300 flex items-center justify-center w-16 h-16 flex-shrink-0 mx-auto">
                      <i class="bx bx-error text-3xl"></i>
                    </div>
                    <div class="mt-4 md:mt-0 md:ml-6 text-center md:text-left">
                      <p class="font-bold">Ново събитие</p>
                      <p class="text-sm text-gray-700 mt-1">Въведи ново събитие в своя календар.</p>
                    </div>
                  </div>
                  <form on:submit=move |ev| {
                    ev.prevent_default();
              
                        // Extract the current day
                    let today_date = today();
              
                      // Parse the hour string (format "HH:MM")
                    let (parsed_hour, parsed_minute) = hour()
                      .split_once(':')
                      .and_then(|(h, m)| Some((h.parse::<u32>().ok()?, m.parse::<u32>().ok()?)))
                      .unwrap_or((12, 0)); // Default to 12:00 if parsing fails
              
                    // Create a NaiveDateTime using today’s date and parsed time
                    let event_datetime = today_date
                      .and_hms_opt(parsed_hour, parsed_minute, 0)
                      .unwrap_or_else(|| today_date.and_hms(12, 0, 0)); // Fallback to 12:00 if invalid
              
                    new_event.dispatch(model::event::CreateEvent { input: (name(), description(), label(), event_datetime) });
                    }>

                    <div class="input-box">
                        <input bind:value=(name, set_name) type="text" placeholder="Име" required class="ring-1 ring-inset ring-black/10 rounded-lg" />
                    </div>
                    <div class="input-box">
                        <textarea 
                          prop:value=move || description()
                          on:input:target=move |ev| set_description(ev.target().value())
                        placeholder="Описание" rows="6" required class="ring-1 w-full ring-inset ring-black/10 rounded-lg" />
                    </div>
                    <div class="input-box flex flex-row gap-2">
                        <select class="appearance-none ring-1 ring-inset ring-black/10 rounded-lg"
                          on:change:target=move |ev| {
                          set_label(ev.target().value().parse().unwrap());
                          }
                          prop:value=move || label().to_string()
                          required
                          >
                            <option value="" selected disabled>Избери предмет</option>
                            <option value="Математика">Математика</option>
                            <option value="български език">Български език</option>
                            <option value="география">География</option>
                        </select>
                        <input bind:value=(hour, set_hour) type="time" id="time" class="ring-1 ring-inset ring-black/10 leading-none text-gray-900 text-sm rounded-lg focus:ring-blue-500 block w-full !p-2.5" min="09:00" max="18:00" value="00:00" required />
                    </div>
                <div class="text-center mt-4">
                  <button type="submit" class="block w-full px-4 py-3 md:py-2 bg-gray-200 rounded-lg font-semibold text-sm mt-4">Cancel</button>
                </div>
                </form>
              </div>
              </div>
            </div>
        </Portal>
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
