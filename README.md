Hollobock
=========

A 2D top-down shooter I'm writing to learn Rust.

Gameplay
========

The gameplay will probably be a combination between the survival and typ-o-shooter game modes from Crimsonland (http://www.crimsonland.com/). You will face waves on enemies and get weapon upgrades. The game is controlled using WASD for movement and mouse for looking and shooting. When you're out of ammo you have to let everything go and quickly type in the complicated word on the screen in order to make your weapon reload.

Tech and other cool stuff
=========================

I am creating this game using the Rust programming language: https://github.com/mozilla/rust. The main purpose for making this game is learning Rust.

I try adopt some functional thinking when doing this. For example, the updating of entities is free of side effects. The entities in the game always return a new self and cannot directly poke other entities in the world.

I create this game using the Rust SFML bindings, which lets me use the SFML multimedia library in Rust. https://github.com/JeremyLetang/rust-sfml
