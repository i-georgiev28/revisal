use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};


use crate::app::components::cards::Card;

#[island]
pub fn Cardset(
  #[prop(into)] id: String,
) -> impl IntoView {
    view! {
        <section id="cardsets"
        class="ring-1 ring-inset ring-black/10 rounded-lg isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5"
        style="--grid-area:cards">
        <div class="flex cursor-pointer group flex-col md:flex-row md:justify-between mx-1 w-content mb-2">
        <p class="font-medium mr-4 text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
          Части на речта</p>
          <span class="mr-auto mt-2.5 inline-flex self-center items-center rounded-md bg-purple-50 px-2 py-1 text-xs font-medium text-purple-700 ring-1 ring-purple-700/10 ring-inset">Български език</span>
          <p class="mt-1.5 mx-0.5 group-hover:mx-2 duration-100 ease-out transition-all text-xl tracking-tight font-semibold bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">{"Виж повече →"}</p>
        </div>
        <div class="cardsets">
          <Card question="В кое време е глаголът „четях“?" answer="AA"/>
          <Card question="Каква част на речта е думата „ако“?" answer="AA"/>
          <Card question="Какъв вид съюз е „но“?" answer="AA"/>
        </div>
      </section>
      <br /> <br />
      <section id="cardsets"
      class="ring-1 ring-inset ring-black/10 rounded-lg isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5"
      style="--grid-area:cards">
      <div class="flex cursor-pointer group flex-col md:flex-row md:justify-between mx-1 w-content mb-2">
      <p class="font-medium mr-4 text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
        Упражнение</p>
        <span class="mr-auto mt-2.5 inline-flex self-center items-center rounded-md bg-purple-50 px-2 py-1 text-xs font-medium text-purple-700 ring-1 ring-purple-700/10 ring-inset">Български език</span>
        <p class="mt-1.5 mx-0.5 group-hover:mx-2 duration-100 ease-out transition-all text-xl tracking-tight font-semibold bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">{"Виж повече →"}</p>
      </div>
      <div class="cardsets">
        <Card question="Какъв вид съществително е думата „учител“?" answer="AA"/>
        <Card question="Как се нарича неопределената форма на глагола?" answer="AA"/>
        <Card question="Кое е местоимението в изречението: „Той прочете книгата.“?" answer="AA"/>
      </div>
    </section>
    <br /> <br />
    <section id="cardsets"
    class="ring-1 ring-inset ring-black/10 rounded-lg isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5"
    style="--grid-area:cards">
    <div class="flex cursor-pointer group flex-col md:flex-row md:justify-between mx-1 w-content mb-2">
    <p class="font-medium mr-4 text-[2em] leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">
      Граматика</p>
      <span class="mr-auto mt-2.5 inline-flex self-center items-center rounded-md bg-purple-50 px-2 py-1 text-xs font-medium text-purple-700 ring-1 ring-purple-700/10 ring-inset">Български език</span>
      <p class="mt-1.5 mx-0.5 group-hover:mx-2 duration-100 ease-out transition-all text-xl tracking-tight font-semibold bg-gradient-to-r from-[#f26500] via-[#B56BFF] to-[#5bb6ff] text-transparent bg-clip-text">{"Виж повече →"}</p>
    </div>
    <div class="cardsets">
      <Card question="Какъв вид изречение е: „Ще отидем ли на кино?“" answer="AA"/>
      <Card question="Коя е основната форма на глагола в българския език?" answer="AA"/>
      <Card question="Какво е „бързо“ в изречението: „Той бяга бързо.“?" answer="AA"/>
    </div>
  </section>
  
    }
}