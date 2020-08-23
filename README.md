# dnd-buff-tracker
small tool to help keep track of buffs and other status effects

migrations are stored using datetimes:
`mkdir $(date +%Y%m%d_%H%M%S)`

to start a server with autoreload:
`systemfd --no-pid -s http::<PORT> -- cargo watch -x run`

not sure of a better way to stop autoreload but this works:

lookup pid
`lsof -i :<PORT>`

end using pid
`kill -9 PID`

