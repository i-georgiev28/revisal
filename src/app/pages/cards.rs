use leptos::{portal::Portal, prelude::*};
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};

use crate::app::components::cards::*;

#[island]
pub fn Cards() -> impl IntoView {
    let is_modal_open: RwSignal<bool> = RwSignal::new(false);
    let toggle_modal = move |_| {
        is_modal_open.set(!is_modal_open.get())
    };
    view! {
        <main class="antialiased" id="cards">
        <button on:click=toggle_modal id="create-new"
        class=" cursor-pointer hover:bg-white/80 leading-[1.1] tracking-tighter bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text isolate max-w-4 flex p-6 text-lg max-h-4 ring-1 ring-inset ring-black/10 rounded-lg text-center items-center justify-center">
            +
        </button>
            <Cardset id="" />
        </main>

        // <div id="dialog">
  
            // <button class="show-modal open-modal">Open Modal</button>
  <Portal>
            <div id="modal" class=move || {
                if is_modal_open.get() {
                    "open transition-all duration-200"
                } else {
                    "transition-all duration-200"
                }
            }>
            // <div class="content">
            //   <span class="close">{"X"}</span>
            //   <p>Some text in the Modal..</p>
            // </div>
            <div class="relative px-4 min-h-screen flex items-center justify-center">
            <button on:click=toggle_modal class="overlay"></button>
            <div class="bg-white rounded-lg max-w-md mx-auto p-4 fixed inset-x-0 bottom-0 z-50 mb-4 mx-4 relative">
              <div class="flex items-center">
                <div class="rounded-full border border-gray-300 flex items-center justify-center w-16 h-16 flex-shrink-0 mx-auto">
                  <i class="bx bx-error text-3xl"></i>
                </div>
                <div class="mt-4 mt-0 ml-6 text-center text-left">
                  <p class="font-bold">Нов пакет с карти</p>
                  <p class="text-sm text-gray-700 mt-1">Създай нов пакет с карти и организирай своето обучение.</p>
                </div>
              </div>
                <div class="input-box">
                    <input type="text" placeholder="Име" required class="ring-1 ring-inset ring-black/10 rounded-lg" />
                </div>
                <div class="input-box">
                    // <input type="password" placeholder="Парола" required class="ring-1 ring-inset ring-black/10 rounded-lg" />
                    <select class="appearance-none w-full ring-1 ring-inset ring-black/10 rounded-lg">
                        <option value="" selected disabled>Избери предмет</option>
                        <option value="Математика">Математика</option>
                        <option value="български език">Български език</option>
                        <option value="география">География</option>
                    </select>
                </div>
              <div class="text-center text-right mt-4 flex justify-end">
                <button class="block w-full inline-block w-auto px-4 py-3 py-2 bg-gray-200 rounded-lg font-semibold text-sm mt-4
                  mt-0 order-1">Cancel</button>
              </div>
            </div>
            </div>
          </div>
          </Portal>
        // </div>
    }
}