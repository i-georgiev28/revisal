use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};

#[island]
pub fn Schedule() -> impl IntoView {
    view! {
        <section id="schedule" class="isolate flex text-[2em] leading-[1.1] tracking-tighter 
        shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5" style="--grid-area:schedule">
      <div class="absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg [mask:linear-gradient(145deg,black,transparent_66%)] ">
        <div class="absolute -translate-y-1/2 -left-12 -top-14 size-52 blur-2xl bg-[#B56BFF] rounded-full opacity-25"></div>
        <div class="absolute top-12 -left-8  size-24 blur-2xl bg-[#f26500] rounded-full opacity-25 "></div>
        <div class="absolute -top-12 left-16  size-32 blur-2xl bg-[#5bb6ff] rounded-full opacity-25"></div>
      </div>
      <A href="#" attr:class="group flex mx-1 flex-col w-content ">
      <span class="mb-1 basis-full w-full bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">Разписание</span>
      <p class="basis-full mx-0.5 group-hover:mx-2 duration-100 ease-out transition-all text-xl tracking-tight font-semibold bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">{"Виж повече →"}</p>
      // <br />
      // <span class="mb-2 bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">събитие</span>

      </A>

      <ul>
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
        
        <li><i> I </i>
          <span> Methametics </span>
        </li>
      </ul>
      </section>
    }
}