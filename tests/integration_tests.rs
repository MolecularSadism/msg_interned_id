//! Integration tests for the InternedId derive macro.

use bevy::prelude::*;
use msg_interned_id::InternedId;
use serde::{Deserialize, Serialize};

/// Test ID type for spells
#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SpellId(bevy::ecs::intern::Interned<str>);

/// Test ID type for items
#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemId(bevy::ecs::intern::Interned<str>);

/// Test ID type used as a component
#[derive(Component, InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct EntityId(bevy::ecs::intern::Interned<str>);

// ============================================================================
// Core Functionality Tests
// ============================================================================

mod core_functionality {
    use super::*;

    #[test]
    fn test_new_creates_id() {
        let id = SpellId::new("fireball");
        assert_eq!(id.as_str(), "fireball");
    }

    #[test]
    fn test_as_str_returns_correct_value() {
        let id = SpellId::new("ice_blast");
        assert_eq!(id.as_str(), "ice_blast");
    }

    #[test]
    fn test_empty_string() {
        let id = SpellId::new("");
        assert_eq!(id.as_str(), "");
    }

    #[test]
    fn test_unicode_strings() {
        let id = SpellId::new("火球术");
        assert_eq!(id.as_str(), "火球术");
    }

    #[test]
    fn test_special_characters() {
        let id = SpellId::new("spell-with_special.chars!@#$%");
        assert_eq!(id.as_str(), "spell-with_special.chars!@#$%");
    }

    #[test]
    fn test_whitespace_string() {
        let id = SpellId::new("  spaced  ");
        assert_eq!(id.as_str(), "  spaced  ");
    }
}

// ============================================================================
// Interning Behavior Tests
// ============================================================================

mod interning {
    use super::*;

    #[test]
    fn test_same_string_equals() {
        let id1 = SpellId::new("lightning");
        let id2 = SpellId::new("lightning");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_different_strings_not_equal() {
        let id1 = SpellId::new("fire");
        let id2 = SpellId::new("water");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_interned_pointer_equality() {
        // Same string should share the same interned pointer
        let id1 = SpellId::new("shared_spell");
        let id2 = SpellId::new("shared_spell");
        // They should be equal via PartialEq (which compares interned pointers)
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_different_types_have_separate_interners() {
        // Same string in different ID types should work independently
        let spell_id = SpellId::new("potion");
        let item_id = ItemId::new("potion");

        // Both should have the same string value
        assert_eq!(spell_id.as_str(), item_id.as_str());
        assert_eq!(spell_id.as_str(), "potion");
    }
}

// ============================================================================
// Standard Trait Tests
// ============================================================================

mod standard_traits {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_display() {
        let id = SpellId::new("thunder");
        assert_eq!(format!("{}", id), "thunder");
    }

    #[test]
    fn test_debug() {
        let id = SpellId::new("debug_spell");
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("SpellId"));
    }

    #[test]
    fn test_clone() {
        let id1 = SpellId::new("clone_test");
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_copy() {
        let id1 = SpellId::new("copy_test");
        let id2 = id1; // Copy
        assert_eq!(id1, id2); // Both should still be valid
    }

    #[test]
    fn test_hash_in_hashset() {
        let mut set = HashSet::new();
        set.insert(SpellId::new("spell1"));
        set.insert(SpellId::new("spell2"));
        set.insert(SpellId::new("spell1")); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&SpellId::new("spell1")));
        assert!(set.contains(&SpellId::new("spell2")));
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let id1 = SpellId::new("hash_test");
        let id2 = SpellId::new("hash_test");

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_default() {
        let id = SpellId::default();
        assert_eq!(id.as_str(), "");
    }

    #[test]
    fn test_from_str() {
        let id: SpellId = "from_str_test".into();
        assert_eq!(id.as_str(), "from_str_test");
    }

    #[test]
    fn test_from_string() {
        let s = String::from("from_string_test");
        let id: SpellId = s.into();
        assert_eq!(id.as_str(), "from_string_test");
    }

    #[test]
    fn test_deref() {
        let id = SpellId::new("deref_test");
        // Deref to &str
        let s: &str = &*id;
        assert_eq!(s, "deref_test");
    }

    #[test]
    fn test_deref_methods() {
        let id = SpellId::new("hello_world");
        // Can use str methods via deref
        assert!(id.starts_with("hello"));
        assert!(id.ends_with("world"));
        assert_eq!(id.len(), 11);
        assert!(id.contains("_"));
    }
}

// ============================================================================
// Serde Serialization Tests
// ============================================================================

mod serde_tests {
    use super::*;

    #[test]
    fn test_serialize_json() {
        let id = SpellId::new("serialize_test");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"serialize_test\"");
    }

    #[test]
    fn test_deserialize_json() {
        let json = "\"deserialize_test\"";
        let id: SpellId = serde_json::from_str(json).unwrap();
        assert_eq!(id.as_str(), "deserialize_test");
    }

    #[test]
    fn test_roundtrip_json() {
        let original = SpellId::new("roundtrip_spell");
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SpellId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialize_empty_string() {
        let id = SpellId::new("");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"\"");
    }

    #[test]
    fn test_deserialize_empty_string() {
        let json = "\"\"";
        let id: SpellId = serde_json::from_str(json).unwrap();
        assert_eq!(id.as_str(), "");
    }

    #[test]
    fn test_serialize_unicode() {
        let id = SpellId::new("魔法咒语");
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: SpellId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.as_str(), "魔法咒语");
    }

    #[test]
    fn test_serialize_in_struct() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct SpellData {
            id: SpellId,
            power: u32,
        }

        let data = SpellData {
            id: SpellId::new("fireball"),
            power: 100,
        };

        let json = serde_json::to_string(&data).unwrap();
        let deserialized: SpellData = serde_json::from_str(&json).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_serialize_in_vec() {
        let ids = vec![
            SpellId::new("spell1"),
            SpellId::new("spell2"),
            SpellId::new("spell3"),
        ];

        let json = serde_json::to_string(&ids).unwrap();
        let deserialized: Vec<SpellId> = serde_json::from_str(&json).unwrap();
        assert_eq!(ids, deserialized);
    }
}

// ============================================================================
// Bevy Reflection Tests
// ============================================================================

mod reflection_tests {
    use super::*;
    use bevy::reflect::{
        FromReflect, GetTypeRegistration, PartialReflect, Reflect, ReflectKind, TypePath, Typed,
    };

    #[test]
    fn test_type_path() {
        let short = SpellId::short_type_path();
        assert_eq!(short, "SpellId");

        let full = SpellId::type_path();
        assert!(full.ends_with("SpellId"));
    }

    #[test]
    fn test_typed() {
        let type_info = SpellId::type_info();
        assert!(matches!(
            type_info,
            bevy::reflect::TypeInfo::Opaque(_)
        ));
    }

    #[test]
    fn test_reflect_kind() {
        let id = SpellId::new("reflect_test");
        assert!(matches!(id.reflect_kind(), ReflectKind::Opaque));
    }

    #[test]
    fn test_reflect_ref() {
        let id = SpellId::new("reflect_ref_test");
        let reflect_ref = id.reflect_ref();
        assert!(matches!(
            reflect_ref,
            bevy::reflect::ReflectRef::Opaque(_)
        ));
    }

    #[test]
    fn test_clone_value() {
        let id = SpellId::new("clone_value_test");
        let cloned = id.clone_value();
        let downcasted = cloned.try_downcast_ref::<SpellId>().unwrap();
        assert_eq!(*downcasted, id);
    }

    #[test]
    fn test_from_reflect() {
        let id = SpellId::new("from_reflect_test");
        let reflected: &dyn PartialReflect = &id;
        let from_reflect = SpellId::from_reflect(reflected).unwrap();
        assert_eq!(from_reflect, id);
    }

    #[test]
    fn test_reflect_partial_eq() {
        let id1 = SpellId::new("partial_eq_test");
        let id2 = SpellId::new("partial_eq_test");
        let id3 = SpellId::new("different");

        assert_eq!(id1.reflect_partial_eq(&id2), Some(true));
        assert_eq!(id1.reflect_partial_eq(&id3), Some(false));
    }

    #[test]
    fn test_reflect_hash() {
        let id = SpellId::new("hash_test");
        let hash = id.reflect_hash();
        assert!(hash.is_some());
    }

    #[test]
    fn test_apply() {
        let mut id1 = SpellId::new("original");
        let id2 = SpellId::new("updated");

        id1.apply(&id2);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_try_apply_success() {
        let mut id1 = SpellId::new("original");
        let id2 = SpellId::new("updated");

        let result = id1.try_apply(&id2);
        assert!(result.is_ok());
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_try_apply_type_mismatch() {
        let mut spell_id = SpellId::new("spell");
        let item_id = ItemId::new("item");

        let result = spell_id.try_apply(&item_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_reflect_set() {
        let mut id1 = SpellId::new("original");
        let id2 = Box::new(SpellId::new("updated"));

        let result = id1.set(id2);
        assert!(result.is_ok());
        assert_eq!(id1.as_str(), "updated");
    }

    #[test]
    fn test_debug_format() {
        let id = SpellId::new("debug_spell");
        let mut output = String::new();
        use std::fmt::Write;
        write!(&mut output, "{:?}", id.as_partial_reflect()).unwrap();
        assert!(output.contains("SpellId"));
        assert!(output.contains("debug_spell"));
    }

    #[test]
    fn test_get_type_registration() {
        let registration = SpellId::get_type_registration();
        assert!(registration.data::<bevy::reflect::ReflectFromReflect>().is_some());
        assert!(registration.data::<bevy::reflect::ReflectFromPtr>().is_some());
        assert!(registration.data::<bevy::prelude::ReflectDefault>().is_some());
    }

    #[test]
    fn test_into_any() {
        let id = SpellId::new("any_test");
        let boxed: Box<dyn Reflect> = Box::new(id);
        let any = boxed.into_any();
        let downcasted = any.downcast::<SpellId>().unwrap();
        assert_eq!(downcasted.as_str(), "any_test");
    }

    #[test]
    fn test_as_any() {
        let id = SpellId::new("as_any_test");
        let any = id.as_any();
        let downcasted = any.downcast_ref::<SpellId>().unwrap();
        assert_eq!(downcasted.as_str(), "as_any_test");
    }

    #[test]
    fn test_try_into_reflect() {
        let id = SpellId::new("try_into_test");
        let boxed: Box<dyn PartialReflect> = Box::new(id);
        let result = boxed.try_into_reflect();
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_as_reflect() {
        let id = SpellId::new("try_as_test");
        assert!(id.try_as_reflect().is_some());
    }

    #[test]
    fn test_into_partial_reflect() {
        let id = SpellId::new("into_partial_test");
        let boxed: Box<SpellId> = Box::new(id);
        let partial: Box<dyn PartialReflect> = boxed.into_partial_reflect();
        let downcasted = partial.try_downcast_ref::<SpellId>().unwrap();
        assert_eq!(downcasted.as_str(), "into_partial_test");
    }
}

// ============================================================================
// ECS Component Tests
// ============================================================================

mod ecs_tests {
    use super::*;

    #[test]
    fn test_as_component() {
        let mut world = World::new();
        let entity = world.spawn(EntityId::new("player_1")).id();

        let id = world.get::<EntityId>(entity).unwrap();
        assert_eq!(id.as_str(), "player_1");
    }

    #[test]
    fn test_query_component() {
        let mut world = World::new();
        world.spawn(EntityId::new("entity_a"));
        world.spawn(EntityId::new("entity_b"));

        let mut query = world.query::<&EntityId>();
        let ids: Vec<&str> = query.iter(&world).map(|id| id.as_str()).collect();

        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"entity_a"));
        assert!(ids.contains(&"entity_b"));
    }

    #[test]
    fn test_multiple_components_same_entity() {
        #[derive(Component, InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct NameId(bevy::ecs::intern::Interned<str>);

        #[derive(Component, InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct CategoryId(bevy::ecs::intern::Interned<str>);

        let mut world = World::new();
        let entity = world
            .spawn((NameId::new("sword"), CategoryId::new("weapon")))
            .id();

        let name = world.get::<NameId>(entity).unwrap();
        let category = world.get::<CategoryId>(entity).unwrap();

        assert_eq!(name.as_str(), "sword");
        assert_eq!(category.as_str(), "weapon");
    }
}

// ============================================================================
// Edge Case Tests
// ============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_very_long_string() {
        let long_string = "a".repeat(10000);
        let id = SpellId::new(&long_string);
        assert_eq!(id.as_str(), long_string);
    }

    #[test]
    fn test_null_byte_in_string() {
        let id = SpellId::new("hello\0world");
        assert_eq!(id.as_str(), "hello\0world");
    }

    #[test]
    fn test_newlines_in_string() {
        let id = SpellId::new("line1\nline2\r\nline3");
        assert_eq!(id.as_str(), "line1\nline2\r\nline3");
    }

    #[test]
    fn test_many_ids_same_string() {
        let ids: Vec<SpellId> = (0..1000).map(|_| SpellId::new("same_spell")).collect();

        // All should be equal
        for id in &ids {
            assert_eq!(id.as_str(), "same_spell");
        }

        // All should equal each other
        let first = ids[0];
        for id in &ids {
            assert_eq!(*id, first);
        }
    }

    #[test]
    fn test_many_different_ids() {
        let ids: Vec<SpellId> = (0..1000)
            .map(|i| SpellId::new(&format!("spell_{}", i)))
            .collect();

        // All should be unique
        let mut unique = std::collections::HashSet::new();
        for id in &ids {
            unique.insert(*id);
        }
        assert_eq!(unique.len(), 1000);
    }
}

// ============================================================================
// Thread Safety Tests
// ============================================================================

mod thread_safety {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_creation() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let ids: Vec<SpellId> = (0..100)
                        .map(|j| SpellId::new(&format!("spell_{}_{}", i, j)))
                        .collect();
                    ids
                })
            })
            .collect();

        let all_ids: Vec<SpellId> = handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect();

        assert_eq!(all_ids.len(), 1000);
    }

    #[test]
    fn test_concurrent_same_string() {
        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let ids: Vec<SpellId> =
                        (0..100).map(|_| SpellId::new("shared_spell")).collect();
                    ids
                })
            })
            .collect();

        let all_ids: Vec<SpellId> = handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect();

        // All should be equal
        let first = all_ids[0];
        for id in &all_ids {
            assert_eq!(*id, first);
        }
    }

    #[test]
    fn test_shared_across_threads() {
        let id = Arc::new(SpellId::new("shared_id"));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let id_clone = Arc::clone(&id);
                thread::spawn(move || {
                    assert_eq!(id_clone.as_str(), "shared_id");
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
