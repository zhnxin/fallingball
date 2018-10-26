# Falling Ball

## Description

A shot game developped by ggez(a 2d game engine).

## Target

The following picture is the expected UI of this game.
![game_image1](./images/game_image.png)


## issue

- [x] Data Bar
- [x] Timer Bar
- [x] Ball
- [ ] Box
- [x] Power Recorder
- [ ] Main State

## examples

You can use `cargo run --exampel <target>` to view the completed material

## ball

Flying the ball by the direction you click.

![ball shot](./images/ball_shot.gif)

## bar

- countdown 5 sec after clicked shown in vertical timer bar;
- record the time(less than 5 sec) between left mouse button down and up;
- show the Y coordinate over the windows hight in vertical bar;
- show the X coordinate over the windows hight in horizontal bar;


![bar](./images/bar.gif)