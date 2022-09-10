# 2D_Physics
2D physics, built to learn rust.
***
## What to expect
* Balls can collide with obstacles.
* Balls can collide with other balls.
* Left click can spawns new balls or move ball.
* Right click can attracts all the balls on the cursor or delete a ball.

***
## How to use
If you want to compile it by yourself, you need to install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). 

Then:
```
$ git clone https://github.com/LeszczyTom/2D_Physics.git
$ cd 2D_Physics
$ cargo run
```
Or you can download the latest [release](https://github.com/LeszczyTom/2D_Physics/releases).  
***

### Screenshots
Walls:
![walls](https://user-images.githubusercontent.com/37774352/189477955-66feac85-0793-4574-b087-1776bf4cec40.gif)
Zero gravity:
![gravity](https://user-images.githubusercontent.com/37774352/189477995-cfbcd8c8-d67f-49e7-823f-cb1f01dbc127.gif)
Left click:
![spwan_move](https://user-images.githubusercontent.com/37774352/189478107-8dae563a-218a-4868-8bf4-874e9f86bf81.gif)
Right click:
![right_click](https://user-images.githubusercontent.com/37774352/189478154-88f71254-a9d2-4c31-95f7-11677f6f365f.gif)

## Dependencies
* [druid](https://github.com/linebender/druid): 0.7.0
* [winres](https://github.com/mxre/winres): 0.1
