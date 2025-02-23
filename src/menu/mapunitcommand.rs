use super::BasicMenuItemFields;

#[unity::class("App", "MapUnitCommandMenu")]
pub struct MapUnitCommandMenu { }

#[unity::class("", "MapUnitCommandMenu.TradeMenuItem")]
pub struct TradeMenuItem {
    base: BasicMenuItemFields
}

// Not sure why these don't need to use App namespace?
#[unity::class("", "MapUnitCommandMenu.EngageSummonMenuItem")]
pub struct EngageSummonMenuItem {
    base: BasicMenuItemFields
}

// Not sure why these don't need to use App namespace?
#[unity::class("", "MapUnitCommandMenu.EngageStartMenuItem")]
pub struct EngageStartMenuItem {
    base: BasicMenuItemFields
}
