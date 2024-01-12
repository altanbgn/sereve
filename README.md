# sereve
CLI Tool for SSH address manager


Hello!
The reason why I made this tool is because I work with bunch of servers through ssh connnections and I got tired of memorizing or making a shell script for each and every server just to connect.
So I made this beauty.


This CLI tool uses OpenSSH program so make sure you install that. Should be installed by default on most systems.


## Installation
Available and tested only on mac for now.
```
brew install sereve
```

## Documentation
For help obviously
```
sereve help
```


To add server to your list
```
sereve add --name <NAME> --username <USERNAME> --host <HOST> --port <PORT>
```


To view list of servers you added
```
sereve list
```


To connect to your added server. ID will be displayed on list
```
sereve connect --id <ID>
```


To remove address from your list
```
sereve remove --id <ID>
```
