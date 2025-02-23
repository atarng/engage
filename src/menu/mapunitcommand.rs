use super::BasicMenuItemFields;

#[unity::class("App", "MapUnitCommandMenu")]
pub struct MapUnitCommandMenu { }

#[unity::class("", "MapUnitCommandMenu.TradeMenuItem")]
pub struct TradeMenuItem {
    base: BasicMenuItemFields
}

#[unity::class("", "MapUnitCommandMenu.SkillAttackMenuItem")]
pub struct SkillAttackMenuItem {
    base: BasicMenuItemFields
}

#[unity::class("", "MapUnitCommandMenu.EngageActionBaseMenuItem")]
pub struct EngageActionBaseMenuItem {
    base: BasicMenuItemFields
}

// EngageActionBaseMenuItem
#[unity::class("", "MapUnitCommandMenu.AttackMenuItem")]
pub struct AttackMenuItem {
    base: BasicMenuItemFields
}

#[unity::class("", "MapUnitCommandMenu.EngageAttackMenuItem")]
pub struct EngageAttackMenuItem {
    base: BasicMenuItemFields
}

// JumpToMenuItem
#[unity::class("", "MapUnitCommandMenu.EngageCommandMenuItem")]
pub struct EngageCommandMenuItem
{
    base: BasicMenuItemFields
}

// EngageCommandMenuItem
#[unity::class("", "MapUnitCommandMenu.EngageStartMenuItem")]
pub struct EngageStartMenuItem {
    base: BasicMenuItemFields
}

// EngageActionBaseMenuItem
#[unity::class("", "MapUnitCommandMenu.EngageSummonMenuItem")]
pub struct EngageSummonMenuItem {
    base: BasicMenuItemFields
}
