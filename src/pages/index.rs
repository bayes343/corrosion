use crate::lib::domain::{ Page, Elements, Content, Events };
use crate::components::{ header };
use crate::scripts::greet;
use crate::styles::{ shared };

pub fn index() -> Page {
  Page {
    path: format!("index"),
    name: format!("Home"),
    elements: vec![
      Elements::Component(header(
        format!("Home"),
        format!("Experimental frontend framework using the rust language.")
      )),
      Elements::Custom(
        format!("div"),
        Content::InnerHtml(vec![
          Elements::Paragraph(
            Content::InnerHtml(vec![
              Elements::Text(format!("This text is ")),
              Elements::Custom(format!("b"), Content::InnerText(format!("bold")), None),
              Elements::Text(format!(".")),
            ])
          )
        ]),
        None
      ),
      Elements::Button(Content::InnerText(format!("Greet!")), Events::OnClick(greet()))
    ],
    styles: vec![
      shared()
    ],
    script: None
  }
}
