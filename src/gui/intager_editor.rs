use macroquad::prelude::*;
use macroquad::ui::*;

pub fn intager_editor(value: &mut usize, label: &str, ui: &mut Ui) -> bool
{
    const SPACER: f32 = 3.0;
    const PLUS: &str = "+";
    const MINUS: &str = "-";

    let text = format!("{}: {}", label, value);
    let text_size = ui.calc_size(&text);
    let plus_size = ui.calc_size(PLUS);

    let mut was_changed = false;

    ui.label(None, &text);
    ui.same_line(text_size.x + SPACER);
    if ui.button(None, PLUS) && *value < usize::max_value()
    {
        *value += 1;
        was_changed = true;
    }

    ui.same_line(text_size.x + SPACER + plus_size.x + SPACER);
    if ui.button(None, MINUS) && *value > usize::min_value()
    {
        *value -= 1;
        was_changed = true;
    }

    was_changed
}