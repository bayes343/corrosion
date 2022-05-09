use crate::lib::domain::{ Layout };

pub fn default() -> Layout {
  Layout {
        template: String::from("<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title></title>
</head>
<body>
    <main></main>
</body>
</html>")
    }
}
