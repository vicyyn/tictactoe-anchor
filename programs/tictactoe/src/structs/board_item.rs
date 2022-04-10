use crate::*;

#[derive(Copy,Clone,AnchorSerialize,AnchorDeserialize,PartialEq)]
pub enum BoardItem {
 BoardItemEmpty,
 BoardItemX,
 BoardItemO,
}

impl Default for BoardItem {
    fn default() -> Self {
        BoardItem::BoardItemEmpty
    }

}
