use kagura::prelude::*;

pub fn arrow_right(attrs: Attributes, events: Events) -> Html {
    let attrs = attrs.class("icon").class("icon--arrow-right");
    Html::span(attrs, events, vec![])
}

pub fn arrow_bottom(attrs: Attributes, events: Events) -> Html {
    let attrs = attrs.class("icon").class("icon--arrow-bottom");
    Html::span(attrs, events, vec![])
}
