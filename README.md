# How it works

// Todo: Improve this doc (xD)
For starters, bevy_prfb is an simple project for lazy creation of entities based on external files. It works with parent relation and is pretty simple to do new things with this

first: You must implement an format for your data, there is already an example to this [here](examples/custom_prefab.rs)
thats pretty much all you need to do for registering your types, and with it you can use serde or whatever you fell like using

# Ui Prefab
there is an made in API for loading Ui from external files implemented, you can even use your own custom widget with it and your own data, for things like markers or more complex data

For now, thats pretty much all

# Todo
- Improve the docs
- Improve error messages
- Find a way to get AssetServer root asset dir so you don't need to give the path with the asset dir when loading the bytes
- Make new examples
