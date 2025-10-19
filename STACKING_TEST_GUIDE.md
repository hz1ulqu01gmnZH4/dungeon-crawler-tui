# Item Stacking & Detailed Descriptions - Test Guide

## What Was Implemented

### ✅ Item Stacking System
- Stackable items automatically merge when picked up
- Stack quantity displayed in inventory (e.g., "Healing Potion x5")
- Using consumables reduces stack count instead of destroying item
- Max stack limits enforced (Healing Potion: 10 max)

### ✅ Detailed Item Descriptions
- Two-tier description system (short + detailed lore)
- Item categories (Weapon, Armor, Consumable, etc.)
- Rich multi-line descriptions with embedded stats
- Category labels in item details

## How to Test

### Test 1: Item Stacking on Pickup

**Setup:**
1. Start new game
2. Navigate to find healing potions (red `!` symbols)

**Expected Behavior:**
- First potion: "You pick up Healing Potion (qty: 1)."
- Second potion: "You pick up 1 Healing Potion (stack: 2/10)."
- Third potion: "You pick up 1 Healing Potion (stack: 3/10)."
- Inventory shows: `a - Healing Potion x3`

**Verification:**
- Only ONE inventory slot used for all potions
- Stack counter increases with each pickup
- Message shows current stack size (e.g., 3/10)

### Test 2: Inventory Display

**Steps:**
1. Pick up multiple healing potions
2. Press `i` to open inventory
3. Navigate to stacked item

**Expected Display:**

```
┌─ Items (5/26) ────────────────────┐
│ a - Healing Potion x5             │
│ b - Rusty Sword [E]               │
│ c - Wooden Shield                 │
│ d - Leather Armor                 │
└───────────────────────────────────┘
```

**Verification:**
- Stack quantity shown as `x5`
- Equipped items marked with `[E]`
- Total items count includes stacks as 1 item

### Test 3: Detailed Descriptions

**Steps:**
1. Open inventory (`i`)
2. Select any item with arrow keys
3. View Details panel on right side

**Expected for Healing Potion:**

```
┌─ Details ────────────────────────────┐
│ Healing Potion                       │
│                                      │
│ Restores 20 HP when consumed.       │
│                                      │
│ A crimson liquid swirls within this  │
│ small glass vial. The potion glows   │
│ faintly with restorative magic.      │
│ Common among adventurers, these      │
│ potions are brewed by alchemists     │
│ using medicinal herbs and minor      │
│ healing enchantments.                │
│                                      │
│ Effect: Restores 20 HP instantly.   │
│ Uses: Single use, vial shatters     │
│ after consumption.                   │
│                                      │
│ Category: Consumable                 │
│ Quantity: 5/10                       │
│ Weight: 1                            │
│ Value: 25 gold                       │
│                                      │
│ HP Restore: +20                      │
│ Uses: 1                              │
└──────────────────────────────────────┘
```

**Verification:**
- Multi-line detailed description displays
- Category shown (Consumable, Weapon, Armor)
- Stack quantity: "Quantity: 5/10"
- Stats shown for equipment/consumables

### Test 4: Using Stacked Consumables

**Steps:**
1. Get damaged by fighting a monster
2. Open inventory (`i`)
3. Select stacked healing potion
4. Press `u` to use

**Expected Behavior:**
- First use: "You use Healing Potion. Restored 20 HP."
- Stack reduces: `a - Healing Potion x4` (was x5)
- Item stays in inventory (not removed)
- HP increases by 20

**Continue Using:**
- Use potion 4 more times
- After 5th use: "The item is consumed."
- Item removed from inventory
- Stack completely depleted

**Verification:**
- Each use reduces stack by 1
- Item not removed until stack = 0
- HP restored with each use
- Clear feedback messages

### Test 5: Equipment with Detailed Descriptions

**Steps:**
1. Find Iron Sword in dungeon
2. Open inventory, select sword
3. View detailed description

**Expected for Iron Sword:**

```
Iron Sword

A well-crafted iron blade.

Forged by a skilled blacksmith, this iron
sword features a straight double-edged
blade with a simple crossguard. The metal
has been properly tempered and shows
professional craftsmanship. A reliable
weapon for any warrior.

Damage: +5 Power
Condition: Excellent
Type: One-handed slashing weapon

Category: Weapon
Weight: 5
Value: 50 gold

Slot: Main Hand
Power: +5
```

**Verification:**
- Rich lore description
- Combat stats embedded in description
- Equipment slot and bonuses shown
- Non-stackable (no Quantity field)

### Test 6: Stack Limit Enforcement

**Steps:**
1. Collect 10 healing potions (max stack)
2. Try to pick up 11th potion

**Expected Behavior:**
- First 10 potions stack: `x10`
- 11th potion message: "You pick up Healing Potion (qty: 1)."
- Creates NEW stack in separate slot
- Inventory now shows:
  - `a - Healing Potion x10`
  - `b - Healing Potion x1`

**Verification:**
- Max stack enforced (10/10)
- Overflow creates new stack
- Both stacks independent

## Quick Reference

### Item Types and Stacking

| Item Type | Max Stack | Stackable? |
|-----------|-----------|------------|
| Healing Potion | 10 | ✅ Yes |
| Weapons | 1 | ❌ No |
| Armor | 1 | ❌ No |
| Shields | 1 | ❌ No |

### UI Changes

**Inventory List:**
- Before: `a - Healing Potion`
- After: `a - Healing Potion x5`

**Details Panel:**
- New field: `Category: Consumable`
- New field: `Quantity: 5/10` (if stackable)
- Enhanced: Multi-line detailed descriptions

**Messages:**
- Pickup: "You pick up 1 Healing Potion (stack: 5/10)."
- Use: "You use Healing Potion. Restored 20 HP." (stack reduces)
- Deplete: "The item is consumed." (stack reaches 0)

## Known Limitations

1. **Stack overflow**: When stack is full (10/10), new items create separate stack
2. **No stack splitting**: Can't split a stack of 10 into 5+5
3. **No stack merging UI**: Stacks merge automatically on pickup only

## Technical Implementation

### Components Modified
- `Stackable` component: Tracks quantity
- `ItemData`: Added category, max_stack, detailed_description
- `pickup_system()`: Auto-merge logic
- `use_item_system()`: Stack reduction logic
- `inventory_renderer.rs`: Display stack quantities

### Files Changed
1. `src/ecs/components.rs` - Enhanced ItemData, added Stackable
2. `src/systems/inventory.rs` - Stacking logic
3. `src/systems/item_spawner.rs` - Detailed descriptions
4. `src/ui/inventory_renderer.rs` - UI updates

## Testing Checklist

- [ ] Pick up multiple healing potions (stack merges)
- [ ] Verify `x5` display in inventory
- [ ] Check detailed description shows lore
- [ ] Use potion, verify stack reduces to `x4`
- [ ] Use all potions, verify item removed at 0
- [ ] Pick up weapons/armor (no stacking)
- [ ] Hit stack limit (10/10), overflow creates new stack
- [ ] View equipment detailed descriptions
- [ ] Verify category labels show correctly
- [ ] Check stat bonuses display for equipment

## Manual Testing Commands

When you run the game:

```bash
# Run the game
cargo run

# Once in game:
# - Use hjkl/arrows to navigate
# - Press 'g' on healing potions to pick up
# - Press 'i' to open inventory
# - Use arrow keys to select items
# - Press 'u' to use selected item
# - Fight monsters to take damage first
```

## Success Criteria

✅ **Stacking Works** if:
- Multiple potions merge into single stack
- Stack quantity displayed correctly
- Using item reduces stack by 1
- Item only removed when stack = 0

✅ **Descriptions Work** if:
- Detailed lore text displays in Details panel
- Categories show correctly
- Equipment stats embedded in descriptions
- Stack info (Quantity: 5/10) visible

## Next Steps After Testing

Once stacking is verified, implement:
1. Quick equipment keys (w/W/T)
2. Character screen (@)
3. Stat comparison when equipping
4. Examine system (x)
