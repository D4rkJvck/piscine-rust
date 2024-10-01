pub struct One {
    pub first_layer: Option<Two>
}
pub struct Two {
    pub second_layer: Option<Three>
}
pub struct Three {
    pub third_layer: Option<Four>

}
pub struct Four {
    pub fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        match &self.first_layer {
            None => None,
            Some(two) => match &two.second_layer {
                None => None,
                Some(three) => match &three.third_layer {
                    None => None,
                    Some(four) => match &four.fourth_layer {
                        None => None,
                        Some(value) => Some(*value),
                    }
                }
            }
        }
    }
}
