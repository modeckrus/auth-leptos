use leptos::logging::*;
use leptos::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! { <div>
        <canvas id="myChart"></canvas>
      </div>

      <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>

      <script src="/chart.js"></script>
    }
}
