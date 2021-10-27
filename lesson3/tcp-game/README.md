# Rust TCP Guess Game
This is just a small exercise for **lesson 3 of the 7th substrate Course,** the original requirement is very simple and a bit boring, thus I decide to put some fun into it.

Everyone learn rust starts from a guess number game from  [The Rust Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html), so i decide to change the origin version into TCP server version, just fit the exercise requirement.



## How To Play

###  1. start the server

first you need to clone this repo, and enter into it:

```bash
git clone https://github.com/Arstman/tcp-guess-game-rust
cd tcp-guess-game-rust
```

then run with cargo

```bash
cargo run
```

rust will compiles and when everything goes well, it will start a TCP server, listening on the port `7878`, waiting for clients.

### 2. play with client

you can use `telnet`client  to connect to the server, and play the game, open a new terminal and type:

```bash
telnet 127.0.0.1 7878
```

should be work in any OS since telnet are so common.

when the connects successfully, the client  side will  show a welcome information, read it and play the game.

### 3. How to quit:

**server side**: just normal `Ctrl +C` will close the server.

**client side**:  you can close the client by 2 ways: 

	1.  Press `Enter` without any other inputs will directly quit the game, as  show in welcome message.
	2.   Press `Ctrl + C` and then press `Enter` , will also quit the client.



## Screen-shots

### client side (by telnet)

![client side screenshot](https://raw.githubusercontent.com/Arstman/tcp-guess-game-rust/main/screenshots/ch3_client_sid.png)

### server side

![server side screenshot](https://github.com/Arstman/tcp-guess-game-rust/blob/main/screenshots/ch3_server_sid.png)

