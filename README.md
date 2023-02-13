# partyline

A simple chat server. Designed to be as simple as possible. really, this is just a toy.

> it's like IRC for babies -- bnied

## Quick start

    BIND_ADDR=0.0.0.0:9999 cargo run

This will start up the server on port 9999 and accept outside connections (by default, it binds to
`127.0.0.1:9999`). The protocol is simple:

* it's a line-based protocol. a packet is terminated with a newline character.
* lines that start with a `/` are treated as commands.
* all other input is considered chat and is broadcast to all connected clients.

That's pretty much it.

Put a `welcome.txt` file whereever you run it to print a message when users connect.

### how do you connect?

Well, you have at least 2 options:

* `nc localhost 9999`
* `telnet localhost 9999`

Then just type.

## built-in commands

* `/who` -- print number of users connected to the system
* `/name <new-name>` change your display name
* `/news` -- doesn't do anything
* `/echo <text...>` -- print back the text to your own screen.

## why??

I recently read [this book on Phone Phreaks in the 60s and 70s][1] and it mentioned this concept called the Party
Line where phreaks in the know could dial in and chat with each other. you never knew who was listening or if
anyone who spoke was who they said they were.

I also have been struggling with the fact that computers and the internet just aren't fun anymore. this
project is an attempt to both give me a really small personal project, make something fun and bring back a
little bit of the old magic.

maybe this could be the basis of a MUD or some other fun thing. I dunno.

[1]: <http://explodingthephone.com/> "Exploding the Phone: The Untold Story of the Teenagers and Outlaws Who Hacked Ma Bell"
