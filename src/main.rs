use leptos::prelude::*;

fn main() {
  console_error_panic_hook::set_once();
  // leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> });
  mount_to_body(App_3_2);
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

#[allow(unused)]
#[component]
fn App_3_2() -> impl IntoView {
  let (count, set_count) = signal(0);
  // Dynamic Classes
  let dynamic_classes = view! {
    <button
      on:click=move |_| {
        *set_count.write() += 1;
      }
      class:red=move || count.get() & 1 == 1
    >
      "Click me: "
      {count}
    </button>
    <button
      on:click=move |_| {
        *set_count.write() += 1;
      }
      class=("button-20", move || count.get() & 1 == 1)
    >
      "Click me: "
      {count}
    </button>
    <button
      on:click=move |_| {
        *set_count.write() += 1;
      }
      class=(["button-20", "rounded", "blue"], move || count.get() & 1 == 1)
    >
      "Click me: "
      {count}
    </button>
  };
  // Dynamic Styles
  let dynamic_styles = view! {
    <button
      on:click=move |_| {
        *set_count.write() += 10;
      }
      style="position: absolute"
      style:left=move || format!("{}px", count.get() + 100)
      style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
      style:max-width="400px"
      style=("--columns", move || count.get().to_string())
    >
      "Click to Move"
    </button>
  };
  // Dynamic attributes
  let dynamic_attributes = view! {
    <progress max="50" value=count />
  };
  let _ = view! {
    { dynamic_classes.clone() }
    { dynamic_attributes }
  };
  // Derived Signals
  let double_count = move || count.get() * 2;
  let derived_signals = view! {
    { dynamic_classes.clone() }
    <progress
        max="50"
        // we use it once here
        value=double_count
    />
    <p>
        "Double Count: "
        // and again here
        {double_count}
    </p>
  };
  derived_signals
}
