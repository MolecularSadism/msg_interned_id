//! Comprehensive tests for the InternedId derive macro with Bevy 0.17.

// Create a facade module that mirrors the bevy crate structure
// This allows the generated code (which uses bevy::* paths) to work
mod bevy {
    pub mod ecs {
        pub mod intern {
            pub use bevy_ecs::intern::*;
        }
    }
    pub mod reflect {
        pub use bevy_reflect::*;
    }
    pub mod prelude {
        pub use bevy_ecs::prelude::*;
        pub use bevy_reflect::prelude::*;
    }
}

use bevy::prelude::*;
use bevy::reflect::{
    FromReflect, GetTypeRegistration, PartialReflect, Reflect, ReflectKind, TypePath, Typed,
};
use bevy_ecs::world::World;
use bevy_reflect::TypeRegistry;
use msg_interned_id::InternedId;
use std::collections::{HashMap, HashSet};

/// Test ID type for basic functionality.
#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TestId(bevy::ecs::intern::Interned<str>);

/// Another ID type to verify separate interners.
#[derive(InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct OtherId(bevy::ecs::intern::Interned<str>);

/// ID type with Component derive for ECS integration tests.
#[derive(Component, InternedId, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComponentId(bevy::ecs::intern::Interned<str>);

mod core_functionality {
    use super::*;

    #[test]
    fn test_new_and_as_str() {
        let id = TestId::new("test_value");
        assert_eq!(id.as_str(), "test_value");
    }

    #[test]
    fn test_empty_string() {
        let id = TestId::new("");
        assert_eq!(id.as_str(), "");
    }

    #[test]
    fn test_interning_deduplicates() {
        let id1 = TestId::new("same_value");
        let id2 = TestId::new("same_value");
        // Same string should be interned to same pointer
        assert_eq!(id1, id2);
        // The pointers should be identical (not just equal values)
        assert!(std::ptr::eq(id1.as_str(), id2.as_str()));
    }

    #[test]
    fn test_different_values_are_different() {
        let id1 = TestId::new("value_a");
        let id2 = TestId::new("value_b");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_separate_interners_for_types() {
        // Each ID type should have its own interner
        let test_id = TestId::new("shared_name");
        let other_id = OtherId::new("shared_name");

        // They should both have the same string value
        assert_eq!(test_id.as_str(), other_id.as_str());

        // But they are different types and not comparable
        // (This is a compile-time check, just verify they exist)
        let _ = test_id;
        let _ = other_id;
    }
}

mod standard_traits {
    use super::*;

    #[test]
    fn test_display() {
        let id = TestId::new("display_test");
        assert_eq!(format!("{}", id), "display_test");
    }

    #[test]
    fn test_debug() {
        let id = TestId::new("debug_test");
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("TestId"));
    }

    #[test]
    fn test_from_str_ref() {
        let id: TestId = "from_str".into();
        assert_eq!(id.as_str(), "from_str");
    }

    #[test]
    fn test_from_string() {
        let id: TestId = String::from("from_string").into();
        assert_eq!(id.as_str(), "from_string");
    }

    #[test]
    fn test_deref_to_str() {
        let id = TestId::new("deref_test");
        // Deref allows using &str methods directly
        assert!(id.starts_with("deref"));
        assert!(id.ends_with("test"));
        assert_eq!(id.len(), 10);
    }

    #[test]
    fn test_default() {
        let id = TestId::default();
        assert_eq!(id.as_str(), "");
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let id1 = TestId::new("clone_test");
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_copy() {
        let id1 = TestId::new("copy_test");
        let id2 = id1; // Copy, not move
        let _ = id1; // id1 is still valid
        assert_eq!(id1, id2);
    }
}

mod collections {
    use super::*;

    #[test]
    fn test_hash_map() {
        let mut map: HashMap<TestId, u32> = HashMap::new();
        map.insert(TestId::new("key1"), 100);
        map.insert(TestId::new("key2"), 200);

        assert_eq!(map[&TestId::new("key1")], 100);
        assert_eq!(map[&TestId::new("key2")], 200);
        assert_eq!(map.get(&TestId::new("key3")), None);
    }

    #[test]
    fn test_hash_set() {
        let mut set: HashSet<TestId> = HashSet::new();
        set.insert(TestId::new("item1"));
        set.insert(TestId::new("item2"));
        set.insert(TestId::new("item1")); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&TestId::new("item1")));
        assert!(set.contains(&TestId::new("item2")));
    }

    #[test]
    fn test_vec() {
        let ids: Vec<TestId> = vec![
            TestId::new("first"),
            TestId::new("second"),
            TestId::new("third"),
        ];

        assert_eq!(ids.len(), 3);
        assert_eq!(ids[0].as_str(), "first");
    }
}

mod serde_integration {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct SerdeTestWrapper {
        id: TestId,
        ids: Vec<TestId>,
    }

    #[test]
    fn test_serialize() {
        let id = TestId::new("serialize_test");
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"serialize_test\"");
    }

    #[test]
    fn test_deserialize() {
        let json = "\"deserialize_test\"";
        let id: TestId = serde_json::from_str(json).unwrap();
        assert_eq!(id.as_str(), "deserialize_test");
    }

    #[test]
    fn test_roundtrip() {
        let original = TestId::new("roundtrip_test");
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TestId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_complex_struct_serde() {
        let wrapper = SerdeTestWrapper {
            id: TestId::new("main_id"),
            ids: vec![TestId::new("id1"), TestId::new("id2")],
        };

        let json = serde_json::to_string(&wrapper).unwrap();
        let deserialized: SerdeTestWrapper = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, wrapper.id);
        assert_eq!(deserialized.ids.len(), 2);
    }
}

mod reflection {
    use super::*;

    #[test]
    fn test_typed_type_info() {
        let type_info = TestId::type_info();
        // Should be Opaque type
        assert!(matches!(type_info, bevy_reflect::TypeInfo::Opaque(_)));
    }

    #[test]
    fn test_type_path() {
        let path = TestId::type_path();
        assert!(path.contains("TestId"));

        let short_path = TestId::short_type_path();
        assert_eq!(short_path, "TestId");
    }

    #[test]
    fn test_reflect_kind() {
        let id = TestId::new("reflect_test");
        assert!(matches!(id.reflect_kind(), ReflectKind::Opaque));
    }

    #[test]
    fn test_reflect_ref() {
        let id = TestId::new("reflect_ref_test");
        let reflect_ref = id.reflect_ref();
        assert!(matches!(reflect_ref, bevy_reflect::ReflectRef::Opaque(_)));
    }

    #[test]
    fn test_reflect_mut() {
        let mut id = TestId::new("reflect_mut_test");
        let reflect_mut = id.reflect_mut();
        assert!(matches!(reflect_mut, bevy_reflect::ReflectMut::Opaque(_)));
    }

    #[test]
    fn test_partial_reflect_methods() {
        let id = TestId::new("partial_reflect_test");

        // Test get_represented_type_info
        let type_info = id.get_represented_type_info();
        assert!(type_info.is_some());

        // Test as_partial_reflect
        let partial: &dyn PartialReflect = id.as_partial_reflect();
        assert!(partial.try_downcast_ref::<TestId>().is_some());
    }

    #[test]
    fn test_reflect_methods() {
        let id = TestId::new("reflect_test");

        // Test as_reflect
        let reflect: &dyn Reflect = id.as_reflect();
        assert!(reflect.is::<TestId>());

        // Test as_any
        let any = id.as_any();
        assert!(any.is::<TestId>());
    }

    #[test]
    fn test_from_reflect() {
        let original = TestId::new("from_reflect_test");
        let reflected: &dyn PartialReflect = &original;

        let recreated = TestId::from_reflect(reflected);
        assert!(recreated.is_some());
        assert_eq!(recreated.unwrap(), original);
    }

    #[test]
    fn test_reflect_clone() {
        let id = TestId::new("reflect_clone_test");
        let cloned = id.reflect_clone();

        // The cloned value should be a box containing a TestId
        assert!(cloned.is_ok());
        let cloned_box = cloned.unwrap();
        let downcast = cloned_box.downcast_ref::<TestId>();
        assert!(downcast.is_some());
        assert_eq!(*downcast.unwrap(), id);
    }

    #[test]
    fn test_reflect_hash() {
        let id1 = TestId::new("hash_test");
        let id2 = TestId::new("hash_test");
        let id3 = TestId::new("different");

        let hash1 = id1.reflect_hash();
        let hash2 = id2.reflect_hash();
        let hash3 = id3.reflect_hash();

        assert!(hash1.is_some());
        assert_eq!(hash1, hash2); // Same value should have same hash
        assert_ne!(hash1, hash3); // Different values should have different hash
    }

    #[test]
    fn test_reflect_partial_eq() {
        let id1 = TestId::new("eq_test");
        let id2 = TestId::new("eq_test");
        let id3 = TestId::new("different");

        assert_eq!(id1.reflect_partial_eq(&id2), Some(true));
        assert_eq!(id1.reflect_partial_eq(&id3), Some(false));
    }

    #[test]
    fn test_try_apply() {
        let mut id = TestId::new("original");
        let new_value = TestId::new("updated");

        let result = id.try_apply(&new_value);
        assert!(result.is_ok());
        assert_eq!(id, new_value);
    }

    #[test]
    fn test_apply() {
        let mut id = TestId::new("original");
        let new_value = TestId::new("applied");

        id.apply(&new_value);
        assert_eq!(id, new_value);
    }

    #[test]
    fn test_set() {
        let mut id = TestId::new("original");
        let new_value: Box<dyn Reflect> = Box::new(TestId::new("set_value"));

        let result = id.set(new_value);
        assert!(result.is_ok());
        assert_eq!(id.as_str(), "set_value");
    }

    #[test]
    fn test_debug_format() {
        let id = TestId::new("debug_fmt");
        let mut output = String::new();
        use std::fmt::Write;
        write!(&mut output, "{:?}", id.as_partial_reflect()).ok();
        // The debug output should contain the type name and value
        assert!(output.contains("TestId") || output.contains("debug_fmt"));
    }
}

mod type_registration {
    use super::*;

    #[test]
    fn test_get_type_registration() {
        let registration = TestId::get_type_registration();

        // Should have type data registered
        assert!(registration.data::<bevy_reflect::ReflectFromReflect>().is_some());
        assert!(registration.data::<bevy_reflect::ReflectFromPtr>().is_some());
        assert!(registration.data::<ReflectDefault>().is_some());
    }

    #[test]
    fn test_register_in_type_registry() {
        let mut registry = TypeRegistry::new();
        registry.register::<TestId>();

        // Should be able to look up the type
        let registration = registry.get(std::any::TypeId::of::<TestId>());
        assert!(registration.is_some());
    }

    #[test]
    fn test_reflect_default() {
        let registration = TestId::get_type_registration();
        let reflect_default = registration.data::<ReflectDefault>().unwrap();

        let default_value = reflect_default.default();
        let downcast = default_value.downcast_ref::<TestId>();
        assert!(downcast.is_some());
        assert_eq!(downcast.unwrap().as_str(), "");
    }
}

mod ecs_integration {
    use super::*;

    #[test]
    fn test_component_derive() {
        // ComponentId should work as a Component
        let mut world = World::new();
        let entity = world.spawn(ComponentId::new("entity_id")).id();

        let id = world.get::<ComponentId>(entity).unwrap();
        assert_eq!(id.as_str(), "entity_id");
    }

    #[test]
    fn test_query_components() {
        let mut world = World::new();
        world.spawn(ComponentId::new("first"));
        world.spawn(ComponentId::new("second"));
        world.spawn(ComponentId::new("third"));

        let mut query = world.query::<&ComponentId>();
        let ids: Vec<_> = query.iter(&world).map(|id| id.as_str()).collect();

        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&"first"));
        assert!(ids.contains(&"second"));
        assert!(ids.contains(&"third"));
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn test_unicode_strings() {
        let id = TestId::new("Hello, \u{4e16}\u{754c}!"); // "Hello, 世界!" in Chinese
        assert_eq!(id.as_str(), "Hello, \u{4e16}\u{754c}!");
    }

    #[test]
    fn test_special_characters() {
        let id = TestId::new("special!@#$%^&*()");
        assert_eq!(id.as_str(), "special!@#$%^&*()");
    }

    #[test]
    fn test_whitespace() {
        let id = TestId::new("  spaces  ");
        assert_eq!(id.as_str(), "  spaces  ");
    }

    #[test]
    fn test_newlines() {
        let id = TestId::new("line1\nline2");
        assert_eq!(id.as_str(), "line1\nline2");
    }

    #[test]
    fn test_very_long_string() {
        let long_string = "a".repeat(10000);
        let id = TestId::new(&long_string);
        assert_eq!(id.as_str().len(), 10000);
    }

    #[test]
    fn test_match_pattern() {
        let id = TestId::new("fire");
        let result = match &*id {
            "fire" => 1.5,
            "ice" => 1.2,
            "lightning" => 1.8,
            _ => 1.0,
        };
        assert_eq!(result, 1.5);
    }
}

mod thread_safety {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_interning() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    for j in 0..100 {
                        let id = TestId::new(&format!("id_{}_{}", i, j));
                        assert!(id.as_str().starts_with("id_"));
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_same_string_concurrent() {
        let barrier = Arc::new(std::sync::Barrier::new(10));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    barrier.wait();
                    TestId::new("concurrent_test")
                })
            })
            .collect();

        let ids: Vec<TestId> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All IDs should be equal
        for id in &ids {
            assert_eq!(*id, ids[0]);
        }

        // All should point to the same interned string
        let first_ptr = ids[0].as_str() as *const str;
        for id in &ids {
            assert_eq!(id.as_str() as *const str, first_ptr);
        }
    }
}
