extern crate cursive;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Checkbox, Dialog, EditView, LinearLayout, ListView,
                     SelectView, TextView};

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Dialog::new()
        .title("Please fill out this form")
        .button("Ok", |s| s.quit())
        .content(ListView::new()
            .child("Name", EditView::new().fixed_width(10))
            .child("Receive spam?",
                   Checkbox::new()
                       .on_change(|s, checked| for name in &["email1",
                                                             "email2"] {
                           s.call_on_id(name, |view: &mut EditView| {
                               view.set_enabled(checked)
                           });
                           if checked {
                               s.focus_id("email1").unwrap();
                           }
                       }))
            .child("Email",
                   LinearLayout::horizontal()
                       .child(EditView::new()
                           .disabled()
                           .with_id("email1")
                           .fixed_width(15))
                       .child(TextView::new("@"))
                       .child(EditView::new()
                           .disabled()
                           .with_id("email2")
                           .fixed_width(10)))
            .delimiter()
            .child("Age",
                   SelectView::new()
                       .popup()
                       .item_str("0-18")
                       .item_str("19-30")
                       .item_str("31-40")
                       .item_str("41+"))
            .with(|list| for i in 0..50 {
                list.add_child(&format!("Item {}", i), EditView::new());
            })));

    siv.run();
}
