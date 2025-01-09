use super::BasicMenuItemFields;

#[unity::class("App", "MapUnitCommandMenu")]
pub struct MapUnitCommandMenu { }

#[unity::class("", "MapUnitCommandMenu.TradeMenuItem")]
pub struct TradeMenuItem {
    base: BasicMenuItemFields
}