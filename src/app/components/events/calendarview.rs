use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};
use chrono::{Datelike, Local};

pub fn CalendarView() -> impl IntoView {
    let values = (1..=31).collect::<Vec<_>>();
    let current_day = Local::now().day();
    view! {
        <div id="view">
            <span class="day">Mo</span>
            <span class="day">Tu</span>
            <span class="day">We</span>
            <span class="day">Th</span>
            <span class="day">Fr</span>
            <span class="day">Sa</span>
            <span class="day">Su</span>
            <button class="faded">24</button>
            <button class="faded">25</button>
            <button class="faded">26</button>
            <button class="faded">27</button>
            <button class="faded">28</button>
            {values.iter().map(|&day| {
                let class = if day == current_day { "date current-day" } else if day < current_day { "faded" } else { "date" };

                view! { <A href={day.to_string()} attr:class={class}>{day}</A> }
            }).collect_view()}
            // <button class="date">1</button>
            // <button class="date">2</button>
            // <button class="date">3</button>
            // <button class="date">4</button>
            // <button class="date current-day">5</button>
            // <button class="date">6</button>
            // <button class="date">7</button>
            // <button class="date">8</button>
            // <button class="date">9</button>
            // <button class="date">10</button>
            // <button class="date">11</button>
            // <button class="date">12</button>
            // <button class="date">13</button>
            // <button class="date">14</button>
            // <button class="date">15</button>
            // <button class="date">16</button>
            // <button class="date">17</button>
            // <button class="date">18</button>
            // <button class="date">19</button>
            // <button class="date">20</button>
            // <button class="date">21</button>
            // <button class="date">22</button>
            // <button class="date">23</button>
            // <button class="date">24</button>
            // <button class="date">25</button>
            // <button class="date">27</button>
            // <button class="date">27</button>
            // <button class="date">28</button>
            // <button class="date">29</button>
            // <button class="date">30</button>
        </div>
    }
}