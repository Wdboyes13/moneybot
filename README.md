# MoneyBot
A discord bot for money-related things  

## Info  
The official bot can be added to any server on discord  
https://discord.com/oauth2/authorize?client_id=1436064615728676979  

Commands are:  
- `!work` / `/work` - Does work, gives random amount of money 1-1000 (Cooldown 3secs)  
- `!gamble <amount>` / `/gamble <amount>` - Takes amount away, then if you win will give back double amount  
- `!check` / `/check` - Returns current balance  
- `!top <x>` / `/check <x>` - Returns leaderboard of top x in the server  
- `!github` / `/github` - Returns github info about this bot  
- `!transfer <User> <Amount>` / `/transfer <User> <Amount>` - Transfer Amount from you to User  

## Requirements  
- Make sure `~/.moneybot` directory exists  
- Make sure we can create and modify `~/.moneybot/data.db`  
- The environment variable `MONEYBOT_TOKEN` must be set to a valid discord bot token.  
- SQLite3 system libraries are required for crate `rusqlite`  