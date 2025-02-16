use unity::{
    prelude::*,
    system::{Dictionary, List},
};
use crate::proc::{ProcInstFields, Bindable};
use crate::gamedata::{unit::Unit, item::ItemData};

#[unity::class("App", "CommonRewardSequence")]
pub struct CommonRewardSequence {
    pub proc: ProcInstFields,
    menu: u64,
    pub exp_list: &'static Dictionary<&'static Unit, i32>,
    pub item_reward_list: &'static List<ItemData>,
    pub reward_money : i32,
}

impl Bindable for CommonRewardSequence {}

impl CommonRewardSequence {
    pub fn create_bind<P: Bindable>(parent: &P, exp_list: &Dictionary<&Unit, i32>, item_list: &List<ItemData>, money: i32, is_bg: bool) {
        unsafe { create_common_reward_bind(parent, exp_list, item_list, money, is_bg, None); }
    }
}


#[skyline::from_offset(0x02532f40)]
fn create_common_reward_bind<P: Bindable>(proc: &P, exp: &Dictionary<&Unit, i32>, items: &List<ItemData>, money: i32, is_bg: bool, method_info: OptionalMethod);
