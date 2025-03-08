use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

#[island]
pub fn Events() -> impl IntoView {
    view! {
    <section class="flex flex-col relative p-8
    text-[2em] leading-[1.1] tracking-tighter
    isolate shadow-[0_1px_1px_-0.5px,0_3px_3px_-1.5px,0_6px_6px_-3px,0_12px_12px_-6px,0_24px_24px_-12px,0_48px_48px_-24px] shadow-black/5" style="--grid-area:events">
    <div class="absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg [mask:linear-gradient(145deg,black,transparent_66%)] ">
    </div>

        <div class="-z-10 absolute inset-1 ring-1 ring-inset ring-black/10 rounded-lg bg-gradient-to-tr from-[#f26500] via-[#B56BFF] via-66% to-[#5bb6ff] [mask:linear-gradient(-45deg,black,transparent_95%)] opacity-10"></div>
        <div class="absolute -top-12 -right-12 size-[200px] rounded-full bg-gradient-to-br from-[#f26500] via-[#B56BFF] via-35% to-[#5bb6ff] [mask:radial-gradient(circle_at_75%_25%,transparent_60%,black)] opacity-15"></div>   
        <div class="absolute -top-16 -left-12 -rotate-45 size-[200px] rounded-full bg-gradient-to-br from-[#f26500] via-[#B56BFF] via-35% to-[#5bb6ff] [mask:radial-gradient(circle_at_75%_25%,transparent_60%,black)] opacity-15"></div> 
        <div class="absolute -bottom-12 -right-4 translate-x-1/2 rotate-45 size-[200px] rounded-full bg-gradient-to-br from-[#f26500] via-[#B56BFF] via-35% to-[#5bb6ff] [mask:radial-gradient(circle_at_75%_25%,transparent_60%,black)] opacity-15"></div> 

        <div class="relative z-10">
            <h2 class="text-2xl font-bold mb-6">
                <span class="bg-gradient-to-b from-[#8d8d8d] to-[#111] text-transparent bg-clip-text">Предстоящи събития</span>
            </h2>
            
            <div class="space-y-4">
                <div class="flex items-center gap-4">
                    <div class="bg-gradient-to-r from-[#f26500] to-[#B56BFF] p-[1px] rounded-lg">
                        <div class="bg-white/95 dark:bg-black/95 rounded-lg p-3">
                            <div class="text-base text-gray-600 font-medium">Контролно</div>
                            <div class="text-sm text-gray-600 dark:text-gray-400">Март 16</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
    }
}