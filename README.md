# Sorting Hat discord bot
This bot is really nothing special, focus on simplicity. After a little configuration you instantly can use it.

## Install
1. Create new application on the Discord developer portal
2. Copy the `token`
3. Add the application to your server
4. clone this repository

---

Create and edit `Secrets.toml` in the repository folder:
```env
DISCORD_TOKEN = 'the copied token from the discord portal'
DISCORD_ROLE_ID = 'the role ID you want to give users activating'
API_URL = 'URL of the API what decides if user is eligible'
API_SECRET = 'base64 credentials for basic auth'
```

Run the commands:
```bash
cargo shuttle project start --name my-sorting-hat --idle-minutes 0
cargo shuttle deploy --name my-sorting-hat
```

Alternatively you can create a `Shuttle.toml` with:
```toml
name = "my-sorting-hat"
```

and run:
```bash
cargo shuttle project start --idle-minutes 0
cargo shuttle deploy --allow-dirty
```

## API limitations
The query syntax is `API_URL/{username}`, and it is a GET request.

The bot expects 3 status codes: 401 if unauthorized, 404 if user is not found (or ineligible), 200 if all ok.

### Troubleshooting

In this example we will deploy a Serenity bot with Shuttle that responds to the `!hello` command with `world!`. To run this bot we need a valid Discord Token. To get started log in to the [Discord developer portal](https://discord.com/developers/applications).

1. Click the New Application button, name your application and click Create.
2. Navigate to the Bot tab in the lefthand menu, and add a new bot.
3. On the bot page click the Reset Token button to reveal your token. Put this token in your `Secrets.toml`. It's very important that you don't reveal your token to anyone, as it can be abused. Create a `.gitignore` file to omit your `Secrets.toml` from version control.
4. For the sake of this example, you also need to scroll down on the bot page to the Message Content Intent section and enable that option.

To add the bot to a server we need to create an invite link.

1. On your bot's application page, open the OAuth2 page via the lefthand panel.
2. Go to the URL Generator via the lefthand panel, and select the `bot` scope as well as the `Send Messages` permission in the Bot Permissions section.
3. Copy the URL, open it in your browser and select a Discord server you wish to invite the bot to.

For more information please refer to the [Discord docs](https://discord.com/developers/docs/getting-started) as well as the [Serenity repo](https://github.com/serenity-rs/serenity) for more examples.
