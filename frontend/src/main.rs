use crate::card_list::CardList;

mod props;
mod card_list;
mod card;

fn main() {
    yew::Renderer::<CardList>::new().render();
}
