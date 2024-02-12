

use axum::response::Html;

pub fn page() -> Html<&'static str> {
  Html("
    <nav class=\"navbar navbar-expand-lg navbar-dark bg-dark\">
      <div class=\"container\">
        <a class=\"navbar-brand\" href=\"#\">{\"Yew TODO App\"}</a>
      </div>
    </nav>
  ")
}