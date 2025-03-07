use leptos::prelude::*;
use leptos_meta::*;
use leptos::either::Either;
use leptos_router::{components::*, *};


#[island]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav data-controller="navbar">
	<div id="logo">
		<img src="/revisal.svg" alt="Logo" />
		// <h1>Revisal</h1>
	</div>
	<div id="nav">
		<A attr:class="item" href="/app/dashboard" attr:style="--hue: 200deg">
    		<span class="icon mono" id="blur0" aria-hidden="true">
    		  {"🏠"}
    		</span>
    		<span class="icon mono" aria-hidden="true">
    		  {"🏠"}
    		</span>
    		<span
    		  class="icon midl"
    		  aria-hidden="true"
    		  style="background-image: -moz-element(#blur0)"
    		>
    		  {"🏠"}
    		</span>
    		<span class="icon grey" aria-hidden="true">
    		  {"🏠"}
    		</span>
    		<span class="label">dashboard</span>
  		</A>
  		<A attr:class="item" href="/app/cards" attr:style="--hue: 260deg">
   			<span class="icon mono" id="blur1" aria-hidden="true">
   			  {"🗒️"}
   			</span>
   			<span class="icon mono" aria-hidden="true">
   			  {"🗒️"}
   			</span>
   			<span
   			  class="icon midl"
   			  aria-hidden="true"
   			  style="background-image: -moz-element(#blur1)"
   			>
   			  {"🗒️"}
   			</span>
   			<span class="icon grey" aria-hidden="true">
   			  {"🗒️"}
   			</span>
   			<span class="label">cards</span>
  		</A>
  		<A attr:class="item" href="/app/calendar" attr:style="--hue: 320deg">
    		<span class="icon mono" id="blur2" aria-hidden="true">
    		  {"🔔"}
    		</span>
    		<span class="icon mono" aria-hidden="true">
    		  {"🔔"}
    		</span>
    		<span
    		  class="icon midl"
    		  aria-hidden="true"
    		  style="background-image: -moz-element(#blur2)"
    		>
    		  {"🔔"}
    		</span>
    		<span class="icon grey" aria-hidden="true">
    		  {"🔔"}
    		</span>
    		<span class="label">calendar</span>
  		</A>
  		<A attr:class="item" href="/app/schedule" attr:style="--hue: 30deg">
    		<span class="icon mono" id="blur3" aria-hidden="true">
    		  {"🧭"}
    		</span>
    		<span class="icon mono" aria-hidden="true">
    		  {"🧭"}
    		</span>
    		<span
    		  class="icon midl"
    		  aria-hidden="true"
    		  style="background-image: -moz-element(#blur3)"
    		>
    		  {"🧭"}
    		</span>
    		<span class="icon grey" aria-hidden="true">
    		  {"🧭"}
    		</span>
    		<span class="label">schedule</span>
  		</A>


		// <A href="/app/dashboard">
		// 	<i data-feather="sun"></i>
		// 	<span>
		// 		Home
		// 	</span>
		// </A>
		// <A href="/app/cards">
		// 	<i data-feather="moon"></i>
		// 	<span>
		// 		Cards
		// 	</span>
		// </A>
		// <A href="/app/calendar">
		// 	<i data-feather="star"></i>
		// 	<span>
		// 		Events
		// 	</span>
		// </A>
		// <A href="/app/schedule">
		// 	<i data-feather="star"></i>
		// 	<span>
		// 		Schedule
		// 	</span>
		// </A>
		<span class="slider"></span>
	</div>
	<div id="profile">
		<button class="flex flex-row items-center justify-center bg:transparent hover:bg-transparent/5 rounded-lg gap-2 px-4 py-1">
			<span class="inline text-gray-800"> Tester </span>
			<img src="/profile-icon.png" class="bg-transparent opacity-50 size-10 rounded-full ring-2 ring-white" alt="Profile Icon" />
		</button>
	</div>
</nav>



        // <nav>
        //     <figure>
        //         <img src="" alt="R" />
        //         <figcaption>evisal</figcaption>
        //     </figure>

        //     <TabList selected_tab = selected_page>
        //         <Tab value="dash">Home</Tab>
        //         <Tab value="cards">Cards</Tab>
        //         <Tab value="events">Events</Tab>
        //         <Tab value="schedule">Schedule</Tab>
        //     </TabList>

        //     <div id="profile">
        //         <button>
        //             <img src="#" alt="Profile Icon" />
        //         </button>
        //     </div>
        // </nav>
    }
}