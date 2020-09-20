use kagura::prelude::*;

pub fn div(text: impl Into<String>) -> Html {
    Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
}
