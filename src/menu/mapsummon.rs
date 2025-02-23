use super::BasicMenuItemFields;

#[unity::class("App", "MapSummonMenu")]
pub struct MapSummonMenu { }

// App.MapSummonMenu.SummonColorMenuItem$$.ctor
// Not sure why these don't need to use App namespace?
#[unity::class("", "MapSummonMenu.SummonColorMenuItem")]
pub struct SummonColorMenuItem {
    base: BasicMenuItemFields
}
