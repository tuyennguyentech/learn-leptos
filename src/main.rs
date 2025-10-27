use leptos::prelude::*;

fn main() {
  console_error_panic_hook::set_once();
  // leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> });
  mount_to_body(App_3_1);
}

#[component]
fn App_3_1() -> impl IntoView {
  let (count, set_count) = signal(0);
  view! {
    <button
      on:click=move |_| {*set_count.write() += 1} style="color: red">"Click me: " {count}</button>
    <p>
      "Double count: " {move || count.get() * 2}
    </p>
    <div style="color: red">{count}</div>
  }
}
