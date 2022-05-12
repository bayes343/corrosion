use crate::lib::domain::{ Layout, Elements };
use crate::components::{ nav };

pub fn default() -> Layout {
  Layout {
        template: Box::new(|| format!("<!DOCTYPE html>
<html lang=\"en\">
<head>
    <base href=\"http://127.0.0.1:5500/public/\">
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <link rel=\"icon\" type=\"image/x-icon\" href=\"rust.png\">
    <title></title>
</head>
<body>
    {nav}
    <main></main>
    {footer}
</body>
</html>",
        nav = Elements::Component(nav()).render(),
        footer = format!("<footer>footer goes here</footer>")))
    }
}
