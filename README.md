# Treevolution

## Overview

This crate is about a hobby project of mine, published with the hopes that someone finds it as interesting as I do.
It's all about the simulation of plant growth and mutation. I am using a real simulated genome which gets altered by mutation.
In this first Version you can hopefully see the potential of this simulation, but keep in mind that it is far from done.

## Usage

The usage is very simple, first a new habitat needs to be created, the only parameter needed is the size of the grid:

````doctestinjectablerust
let mut habitat = Habitat::new(IVec2::new(256, 32));
````

After that you want to set a minimum plant count for your habitat, so you dont have to spawn the first plants by hand:
````doctestinjectablerust
habitat.set_minimum_plants(20);
````

The only thing left is to call the habitats update() function in a loop, each update represents one time step. 

## Parameters

I have built in a lot of adjustable hyperparameters, e.g. for controlling energy gain/consumption or lifetime of plants.
There are currently around 20 parameters with a lot more to come.
I am planning to make them openly changeable and better documented, because seeing the impact of different environment constraints is one of the main reasons i am working on this project.

## Potential Extensions

* a gravity-like constraint
* mouse support for the included grid window (selecting plants and cells)
* a kind of debugging to see the "stats" of different plants
* a lot of new constraints to play around with