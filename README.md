# Discord Bot  

This so far is a very simple Discord bot using the [https://github.com/serenity-rs/serenity](serenity-rs/serenity)
framework.

The initial purpose of this bot is to allow people on my server to get the status of or restart a Valheim server 
running on my local network. While I'm creating this for Valheim it could really be used for any systemd service
running.

## Current State  

This is still very much under construction. The commands so far are just stubs that reply to the calls with an under 
construction message.

## Run  

This requires the `BOT_TOKEN` environment variable to be set to a Discord bot user. The bot needs to have the 
`applications.commands` permission (I'm pretty sure). The application currently expects a `.env` file to be in the 
current directory to provide this, I'm sure I'll make this optional at some point.

If running/testing locally you can just `cargo run`. You should be able to `cargo install` if you're building on
the same machine that you're running on. `cargo build` to generate a binary.