CODE
Pupik is a standalone game written purely in Rust lang using the Bevy game engine 0.9. It's main purpose was for me to learn more about Rust and get into the flow of the language. Bevy doesn't have a graphic editor yet (17.02.2023), so everything is code.

The inspiration behind this game was originally a Pou minigame where the player tilts the phone to make Pou go right or left as he jumps, landing on platforms and getting higher and higher. I wanted to create a game similarly simple to control on PC.

-----------------------------------

This project uses Bevy Rapier 2D as its physics engine, I also tried Heron, but I just couldn't make collisions work quite as I imagined (which I later changed, but I alredy got used to rapier and its flexibility). I also use bevy_kira_audio 0.13 for audio (I plan way more complex audio systems later on and Bevy's audio is not quite there yet) and Bevy_mouse_tracking_plugin 0.5.3 for way easier mouse tracking than what Bevy provides.

I was also considering piston game engine, which was way simpler and I did not intend to make it into a big game, but that came to a  quick end as it couldn't lock the cursor inside window and i found moving with WASD lame. Later I found out about Fyrox, a lot more mature game engine, but it was simply too late. 

All graphic design is self made and took way too long, font is free for personal use from https://www.dafont.com/love-letters.font and music was made by Vojtěch Klhůfek. 

Bevy game engine uses Entity Component System, where Entities are unique things" that are assigned groups of Components, which are then processed using Systems. In Bevy one filters entities based on their components using Queries (there you can say which fields does the entity have using With or doesnt have using Without and more), accesses resources through Res(Mut) keyword and spawns/despawns entities using Commands. I decided to split the game into multiple modules, and if they have a System that should be called by the app, the module also includes a similarly named plugin. 

-----------------------------------

Some technical details:

The game isn't fullscreen because it is meant as a small game on the side, I don't want to pretend it's worthy of so much space.

It's infinite, objects spawn (and despawn) just outside player's view.

Instead of making the unicorn go up, I decided to make everything go down, simply because I was scared moving the unicorn could behave weird. I could either just change his y coordinate, but that could brake collisions, or give him vertical acceleration, that would just not work because of strong linear_damping (which is present cause it makes all movements smoother).

I decided to make the planes fall slower just to give an illusion of the flying instead of staying still/falling. They are also the only enemy that spawns when an invisible sensor is activated, that was because originally i thought i was going to make the unicorn fly and calculating when a plane should fly over and where could get compliated. Also, I wanted to learn how sensors work.

Once player gets too close to a black hole, he just moves into the middle of it. That is because gravity doesn't really work once you get too close to a nonsolid object with mass, it yeets a lot. That is exactly the reason why I didnt implement some kind of White Holes which would yeet player across the screen if he got close.

The homing rainbow is implemented as a singular small rectangular piece of rainbow, lets call it Rect, that is pulled towards player, similar implementation as blackhole pulling the player closer (there is a lower bound on pull force here tho). On the coordinates of Rect more Rects spawn with the same rotation, but they stay in place and have shorter lifespan. Its more of a rectangle with rainbow trail.

-----------------------------------

At first, pre-startup systems load in all spritesheets. Then score, speed counter and camera are spawned and audio starts playing. The game starts are main menu, where a there is credits, a clickable background (once clicked, the game loads) and the tutorial button  spawned on enter. Clouds start spawning in the background and the unicorn is visible as well.

If the tutorial button is pressed, tutorial page loads. In the tutorial page, covered in the tutorial.rs, there is only a long text explaining the game, dark background and a back button. In the tutorial page unicorn despawns.
 
If the player clicks anywhere else in the main menu, game starts. The cursor is hidden, player can now move the unicorn by mouse and hide him by clicking left mouse button. When hidden, his hitbox is off, so he cannot collide with anything. Also score counter and energy counter pop in. Once the energy counter reaches 0, unicorn cant hide anymore. Also enemies start spawning and falling from the sky. The later you are in the game, the faster they fall. The unicorn has 3 hp, once he gets below 0, he dies. His current hp is indicated by his sprite, as his cloud gets damaged for each lost hp. 

Once dead, the cursor appears again, the screen turns darker and final score is showed, together with a "return to menu" button. Enemies stop spawning now, collisions stop registering, the unicorn is again incontrollable.

Once player returns to the main menu, score is reset, player is respawned, speed is reset and all enemies are despawned.

-----------------------------------

A list of all functions implemented in each module can be found in main.rs labeled by // --- MODULE DECLARATION --- , here is a quick overview:

audio.rs - audio
basic.rs, blackhole.rs, energybars.rs, homing.rs, plane.rs, planet.rs - Types of enemies and their functionalities
clouds.rs - background clouds
collisions - collision system
cursor.rs - hiding and unhiding cursor
endscreen.rs, mainmenu.rs, tutorial_screen.rs, - few different menus / game states.
falling.rs - implements downwards translation
main.rs - where all plugins are inserted, camera is spawned, images are loaded, objects get animated and window is set up.
map_layout.rs - future map layout, for now spawns enemies randomly
player.rs - player and things around him, forces affecting him etc.
speed.rs - speed scaling, reseting etc.
text.rs - ingame text (as opposed to menu text).

-----------------------------------

GAME

To play the game, go to binaries/pupik.exe .
Tutorial is in the game itself, in the main menu click on the "How to play" button.
