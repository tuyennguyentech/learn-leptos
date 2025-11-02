use leptos::prelude::*;

#[allow(unused, dead_code)]
#[component]
pub fn App() -> impl IntoView {
  let (count, set_count) = signal(0);
  view! {
    <button
      on:click=move |_| *set_count.write() += 1
    >
      "Click me"
    </button>
    <ProgressBar progress=count />
  }
}

#[component]
fn ProgressBar(
  #[prop(optional)] max: u16, // default is `Default::default()` value
  #[prop(default = 100)] default_prop: u16,
  progress: ReadSignal<i32>,
) -> impl IntoView {
  view! {
    <progress
      max=default_prop
      value=progress
    />
  }
}
