use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

#[island]
pub fn Homework() -> impl IntoView {
    view! {
        <section id="homework" class="isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5" style="--grid-area:homework">
        <div class="absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg [mask:linear-gradient(145deg,black,transparent_66%)] ">
        <div class="absolute -translate-y-1/2 -top-1 -left-12 blur-2xl size-[200px] rounded-full bg-[#B56BFF] opacity-25"></div>
        <div class="absolute -translate-y-1/2 top-1/2 -right-12 blur-2xl size-[120px] rounded-full bg-[#5bb6ff] opacity-25"></div>
        <div class="absolute translate-y-1/2 bottom-1 -left-12 blur-2xl size-[96px] rounded-full bg-[#f26500] opacity-25"></div>
        </div>
      //  <div class="z-10 p-4 size-[160px]  rounded-full bg-[linear-gradient(rgb(255_255_255/0.01),rgb(244_244_244/0.1))] shadow-[inset_0_1px_0_#FCFCFC,inset_0_0_2px_2px_rgb(255_255_255/0.5),0_1px_1px_rgb(0_0_0/0.06)] ring-1 ring-black/[0.02]">
      //    <div class="size-full rounded-full bg-white/[0.01] shadow-[inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.25),inset_10px_11px_16px_rgba(180,_106,_255,_0.3),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.2),inset_0px_6px_10px_rgba(0,_0,_0,_0.15)] p-2" >
      //      <div class="size-full rounded-full bg-gradient-to-b from-[#111] to-[#222] shadow-[0px_8px_12px_rgba(0,_0,_0,_0.15),inset_0px_0px_12px_rgba(255,_255,_255,_0.6),inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.4),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.4),inset_6px_7px_15px_rgba(180,_106,_255,_0.6),_inset_30px_30px_0px_-8px_rgba(255,_255,_255,_0.08)]"></div>
      //    </div>
      //  </div>
      // <div class="stats">
      <p class="title mx-1 w-content mb-2 font-medium text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
      Напредък</p>
      // <h2 class="widget__subtitle">Noise Cancellation</h2>
      <div class="subtitle ">
        <p class="mx-2 mb-2 w-content font-normal text-sm leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        Преговорени 
        <span class="font-extrabold mx-1 pr-px bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">
        7/10 </span>
        предмета.
        </p>
        <p class="mx-2 mb-2 w-content font-normal text-sm leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        Оставащи 
        <span class="font-extrabold mx-1 pr-px bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">
        3/5 </span>
        събития.
        </p>
        <p class="mx-2 mb-2 w-content font-normal text-sm leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        Преговорен 
        <span class="font-extrabold mx-1 pr-px bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">
        90% </span>
        от материала.
        </p>
        <p class="mx-2 mb-2 w-content font-normal text-sm leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        <span class="font-extrabold mx-1 pr-px bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">
        15/20 </span>
        флашкарти прегледани.
        </p>
      </div>
      <div class="widget__bars mx-2" id="battery-bars">
        <div class="widget__bar inset-0 ring-1 ring-inset ring-black/10 rounded-lg">
          <div class="widget__bar-fill widget__bar-fill--green" style="height: 60%;"></div>
                          <p class="widget__bar-percent leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">30%</p>
            // <div class="widget__bar-icon" title="Something" aria-label="Something">
            //   I
            // </div>
            <div class="relative w-9 h-9 place-items-center opacity-50 bg-black z-10 rounded-full bg-[linear-gradient(rgb(255_255_255/0.01),rgb(244_244_244/0.1))] shadow-[inset_0_1px_0_#FCFCFC,inset_0_0_2px_2px_rgb(255_255_255/0.5),0_1px_1px_rgb(0_0_0/0.06)] ring-1 ring-black/[0.02]">
              <div class="size-full rounded-full bg-black opacity-75 shadow-[inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.25),inset_10px_11px_16px_rgba(180,_106,_255,_0.3),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.2),inset_0px_6px_10px_rgba(0,_0,_0,_0.15)] p-2" >
                <div class="size-full rounded-full bg-gradient-to-b from-[#111] to-[#222] shadow-[0px_8px_12px_rgba(0,_0,_0,_0.15),inset_0px_0px_12px_rgba(255,_255,_255,_0.6),inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.4),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.4),inset_6px_7px_15px_rgba(180,_106,_255,_0.6),_inset_30px_30px_0px_-8px_rgba(255,_255,_255,_0.08)]"></div>
              </div>
            </div>
        </div>
        <div class="widget__bar inset-0 ring-1 ring-inset ring-black/10 rounded-lg">
          <div class="widget__bar-fill widget__bar-fill--red" style="height: 9%;"></div>
                          <p class="widget__bar-percent leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">30%</p>
                          <div class="relative w-9 h-9 place-items-center opacity-50 bg-black z-10 rounded-full bg-[linear-gradient(rgb(255_255_255/0.01),rgb(244_244_244/0.1))] shadow-[inset_0_1px_0_#FCFCFC,inset_0_0_2px_2px_rgb(255_255_255/0.5),0_1px_1px_rgb(0_0_0/0.06)] ring-1 ring-black/[0.02]">
                          <div class="size-full rounded-full bg-black opacity-75 shadow-[inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.25),inset_10px_11px_16px_rgba(180,_106,_255,_0.3),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.2),inset_0px_6px_10px_rgba(0,_0,_0,_0.15)] p-2" >
                            <div class="size-full rounded-full bg-gradient-to-b from-[#111] to-[#222] shadow-[0px_8px_12px_rgba(0,_0,_0,_0.15),inset_0px_0px_12px_rgba(255,_255,_255,_0.6),inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.4),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.4),inset_6px_7px_15px_rgba(180,_106,_255,_0.6),_inset_30px_30px_0px_-8px_rgba(255,_255,_255,_0.08)]"></div>
                          </div>
                        </div>
        </div>
        <div class="widget__bar inset-0 ring-1 ring-inset ring-black/10 rounded-lg">
          <div class="widget__bar-fill widget__bar-fill--yellow" style="height: 28%;"></div>
                          <p class="widget__bar-percent leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">30%</p>
                          <div class="relative w-9 h-9 place-items-center opacity-50 bg-black z-10 rounded-full bg-[linear-gradient(rgb(255_255_255/0.01),rgb(244_244_244/0.1))] shadow-[inset_0_1px_0_#FCFCFC,inset_0_0_2px_2px_rgb(255_255_255/0.5),0_1px_1px_rgb(0_0_0/0.06)] ring-1 ring-black/[0.02]">
                          <div class="size-full rounded-full bg-black opacity-75 shadow-[inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.25),inset_10px_11px_16px_rgba(180,_106,_255,_0.3),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.2),inset_0px_6px_10px_rgba(0,_0,_0,_0.15)] p-2" >
                            <div class="size-full rounded-full bg-gradient-to-b from-[#111] to-[#222] shadow-[0px_8px_12px_rgba(0,_0,_0,_0.15),inset_0px_0px_12px_rgba(255,_255,_255,_0.6),inset_12px_-11px_20px_2px_rgba(242,_101,_13,_0.4),inset_-9px_-5px_17px_6px_rgba(88,_181,_255,_0.4),inset_6px_7px_15px_rgba(180,_106,_255,_0.6),_inset_30px_30px_0px_-8px_rgba(255,_255,_255,_0.08)]"></div>
                          </div>
                        </div>
        </div>
      </div>
      // </div>
      </section>
    }
}