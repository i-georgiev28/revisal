use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

#[island]
pub fn Card(
  #[prop(into)] question: String,
  #[prop(into)] answer: String,
) -> impl IntoView {
    view! {
        // <div class="card green shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5">
        // <div class="front">
          // <div class="absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg [mask:linear-gradient(145deg,black,transparent_66%)] ">
            // <div class="absolute -translate-y-1/2 -left-12 -top-14 size-52 blur-2xl bg-[#B56BFF] rounded-full opacity-25"></div>
            // <div class="absolute top-12 -left-8  size-24 blur-2xl bg-[#f26500] rounded-full opacity-25 "></div>
            // <div class="absolute -top-12 left-16  size-32 blur-2xl bg-[#5bb6ff] rounded-full opacity-25"></div>
          // </div>
          // <section class="card-header">
            // <img src="/threedots.svg" />
          // </section>
          // <section class="card-body">
            // <h3 class="tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">{ question } {answer}</h3>
          // </section>
          // <section class="card-footer">
            // <a href="#" class="btn-countdown">{"→"}</a>
          // </section>
        // </div>
      // </div>

      <div class="card green shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5">
      <div class="absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg [mask:linear-gradient(145deg,black,transparent_66%)] ">
        <div class="absolute -translate-y-1/2 -left-12 -top-14 size-52 blur-2xl bg-[#B56BFF] rounded-full opacity-25"></div>
        <div class="absolute top-12 -left-8  size-24 blur-2xl bg-[#f26500] rounded-full opacity-25 "></div>
        <div class="absolute -top-12 left-16  size-32 blur-2xl bg-[#5bb6ff] rounded-full opacity-25"></div>
      </div>
      <section class="card-header">
        <div class="date">
          {""}
        </div>
        <img src="/threedots.svg" clas="hover:cursor-pointer" />
      </section>
      <section class="card-body">
        // <h3 class="tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">{question}</h3>
        <p>{question}</p>
      </section>
      <section class="card-footer">
        <a href="#" class="btn-countdown">{"→"}</a>
      </section>
    </div>
    }
}