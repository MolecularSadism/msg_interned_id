# msg_interned_id

Derive macro for generating interned string ID types with full Bevy integration.

## Features

- **Zero-cost abstraction**: String comparisons become pointer comparisons
- **Type safety**: Prevents mixing different ID types (e.g., SpellId vs ItemId)
- **Full Bevy integration**: Reflection, serialization, and ECS component support
- **Developer-friendly**: Inspector UI support in development builds
- **Efficient memory**: Identical strings are deduplicated and share memory

## What is String Interning?

String interning is a technique where identical strings are stored only once in memory. When you create an interned string, the system checks if that string already exists. If it does, you get a reference to the existing string; otherwise, a new one is created. This makes:

- **String comparison extremely fast**: Just compare pointers instead of comparing each character
- **Memory usage lower**: No duplicate strings in memory
- **IDs perfect for games**: Great for asset IDs, event types, configuration keys, etc.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
msg_interned_id = "0.1"
bevy = "0.16"
```

## Quick Start

```rust
use msg_interned_id::InternedId;
use bevy::prelude::*;

// Define your ID type
#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SpellId(bevy::ecs::intern::Interned<str>);

// Use it
let fireball = SpellId::new("fireball");
let ice_bolt = SpellId::new("ice_bolt");

// Fast comparison (pointer equality)
assert_eq!(fireball, SpellId::new("fireball"));
assert_ne!(fireball, ice_bolt);

// Access the string value
println!("Casting: {}", fireball); // Prints: "Casting: fireball"
assert_eq!(fireball.as_str(), "fireball");
```

## Usage Examples

### As ECS Component

```rust
use msg_interned_id::InternedId;
use bevy::prelude::*;

#[derive(Component, InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemId(bevy::ecs::intern::Interned<str>);

fn spawn_item(mut commands: Commands) {
    commands.spawn((
        ItemId::new("health_potion"),
        Transform::default(),
    ));
}

fn query_items(q_items: Query<&ItemId>) {
    for item_id in &q_items {
        println!("Found item: {}", item_id);
    }
}
```

### With HashMap/HashSet

```rust
use std::collections::HashMap;
use msg_interned_id::InternedId;
use bevy::prelude::*;

#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EnemyId(bevy::ecs::intern::Interned<str>);

let mut enemy_hp: HashMap<EnemyId, u32> = HashMap::new();
enemy_hp.insert(EnemyId::new("goblin"), 50);
enemy_hp.insert(EnemyId::new("dragon"), 500);

assert_eq!(enemy_hp[&EnemyId::new("goblin")], 50);
```

### Serialization

```rust
use msg_interned_id::InternedId;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(InternedId, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct QuestId(bevy::ecs::intern::Interned<str>);

#[derive(Serialize, Deserialize)]
struct SaveData {
    active_quest: QuestId,
    completed_quests: Vec<QuestId>,
}

// Serializes as JSON:
// {
//   "active_quest": "main_quest",
//   "completed_quests": ["tutorial", "fetch_quest"]
// }
```

### With Match and Deref

```rust
use msg_interned_id::InternedId;
use bevy::prelude::*;

#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DamageType(bevy::ecs::intern::Interned<str>);

fn calculate_damage(damage_type: DamageType, base_damage: f32) -> f32 {
    // Can use as &str through Deref
    match &*damage_type {
        "fire" => base_damage * 1.5,
        "ice" => base_damage * 1.2,
        "lightning" => base_damage * 1.8,
        _ => base_damage,
    }
}
```

## Generated API

For a type `#[derive(InternedId)] pub struct MyId(...)`:

### Methods
- `MyId::new(s: &str) -> Self` - Create ID from string (interns automatically)
- `id.as_str() -> &'static str` - Get the string value

### Trait Implementations
- `Display` - Format as the string value
- `From<&str>` and `From<String>` - Convenient conversions
- `Deref<Target = str>` - Use as string slice with deref coercion
- `Default` - Empty string default
- `Serialize`, `Deserialize` - Serde support (as string)
- Full Bevy reflection hierarchy

### Derive Requirements
You must manually derive: `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Debug`

## Use Cases

Perfect for:
- **Asset identifiers**: SpellId, ItemId, EnemyId, SoundId
- **Configuration keys**: Settings, feature flags
- **Event types**: GameEvent, NetworkMessage
- **State machine states**: PlayerState, AIState
- **Any string-based identifier needing frequent comparison**

## Performance

String interning provides significant performance benefits for ID types:

- **Comparison**: O(1) pointer comparison instead of O(n) string comparison
- **Memory**: Shared storage for duplicate strings
- **Hashing**: Hash the pointer instead of the string content
- **Copy**: Copy a single pointer instead of string data

## Bevy Integration

The generated types work seamlessly with Bevy's systems:

- **Reflection**: Full support for Bevy's reflection system
- **Inspector**: Read-only display in bevy-inspector-egui (with `dev` feature)
- **Serialization**: Works with Bevy scenes and save systems
- **Type Registration**: Automatic registration with `ReflectDefault`

## Best Practices

1. **One interner per ID type**: Each ID type gets its own interner (no cross-contamination)
2. **Use for identifiers**: Best for values compared frequently, not for arbitrary user text
3. **Not for dynamic content**: Interned strings live for the program lifetime
4. **Type safety**: Create separate types (SpellId, ItemId) instead of generic `Id` type

## Comparison with Alternatives

| Approach | Comparison | Memory | Type Safety |
|----------|------------|--------|-------------|
| `String` | O(n) | High (duplicates) | Low |
| `&'static str` | O(n) | Low | Low |
| `InternedId` | O(1) | Low | High |
| `enum` | O(1) | Lowest | Highest (but inflexible) |

Choose `InternedId` when you need the flexibility of strings with the performance of enums.

## Bevy Version Compatibility

| `msg_interned_id` | Bevy |
|-------------------|------|
| 0.1               | 0.16 |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! This crate is part of the [MolecularSadism](https://github.com/MolecularSadism) game development libraries.
