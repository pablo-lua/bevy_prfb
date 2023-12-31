# How it works
- First, we load the bytes from given path
- Then, with the given format, We turn bytes into an `Prefab<PD>` type, with PD being your PrefabData
> You are expect to create your formatter, being it with serde or other things. There is some examples to it [here](examples/custom_prefab.rs)
- Finally, we give the prefab to you, you can do pretty much everything you need with it. 

## PrefabData
The PrefabData is a trait responsible to turn the data into Components and insert them into the entity

The `PrefabData` need two things:
- The function to insert it into the entity
- The function for loading the assets that the data may have

If your type is a simple type, only a simple component, you can implement `IntoComponent` for the type, and IntoComponent will do the work for you
`IntoComponent` need two things:
- The type Component, the type that the impl will be turned in
- the function into_component, to turn it into the component.

Finally, if you have an struct like this:
```rust
struct Foo {
   bar: ()
}
```

You can use the Derive to PrefabData and implement PrefabData for this struct
```rust
#[derive(PrefabData)]
struct Foo {
   bar: ()
}
```
**Note**: All the fields in the struct may implement PrefabData

## Spawning the Prefab
For spawning the prefab, you may first load it with `Commands::load_prefab::<PD, F<PD>>` (you may give the PrefabData type and the used Formatter
Then, you can finally spawn the prefab with Prefab::spawn()

**Note**: We implement Commands with a few functions to this matter. 
- spawn_empty(): This function will spawn the prefab without any given bundle (Note that all the PrefabData will still be loaded)
- spawn(Bundle): In this function, you can give the bundle. That bundle will be given to the root parent.

But what if I have internal assets to load? Well, in that case you need to first prepare the prefab with Prefab::prepare
In this case we implemented Commands with other few functions:
- prepare_and_spawn(): This function will consume the Prefab and load all the sub-assets.
- prepare_and_spawn_prefab_with(Bundle): this function will consume the prefab, spawn it with the given Bundle and load all the sub-assets
- prepare_prefab(): This function will only prepare the prefab so you can spawn easily in the future. An already prepared prefab doesn't need to be prepared again. This will give the prefab back in the form of an LoadedPrefab event
**Note**: If you want to re-use the prefab frequently, you may Clone the Prefab, this is made this way you don't need to reload all the assets.

# Ui Prefab
there is an made in API for loading Ui from external files implemented, you can even use your own custom widget with it and your own data, for things like markers or more complex data

checkout this [example](examples/ui/custom_ui.rs), this is a perfect copy of the bevy ui example.

# Todo
- Improve the docs
- Improve error messages
- Find a way to get AssetServer root asset dir so you don't need to give the path with the asset dir when loading the bytes
- Make new examples


This crate is designed for use with [Bevy](https://github.com/bevyengine/bevy) [the great engine :)]
