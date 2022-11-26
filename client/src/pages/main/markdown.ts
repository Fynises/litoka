const markdown = `
## How to use
---
### Insert into OBS
---
After configuring the Shoutout options, copy the link and insert into an OBS browser source.  
browser source dimensions: \`800x600\`
### Usage
---
Litoka shoutout player is active as long as the browser source is active.  
The shoutout command is run by sending in your twitch chat:  \`!so\` followed by the name of the channel that you're shouting-out.  
**Example:** \`!so litokabot\` (the name of the channel is case-insensitive)  
The command also supports using chat mentions which allows including an \`@\` symbol before the channel name  
**Example:** \`!so @litokabot\`
  
The shoutout player will automatically queue clips if the command was sent in quick succession.
### Understanding filter types
---
Litoka shoutout player supports filtering of clips to be played.  
If filtering is disabled, Litoka will randomly pick a clip from the target channels' entire clip catalog.  

* **Top Viewed:**  
Pick randomly from a range of the channel's top viewed clips

* **Number Recent:**  
Pick randomly from a range of the channel's most recent clips

* **Time:**  
Pick randomly from a set of clips that fit within the date range specified,  
the parameter is the number of days since today.  
For example, setting the parameter to \`90\` will pick clips that were created between now and 90 days in the past.


### Understanding Overrides
---
Litoka shoutout player supports inline overrides when executing the shoutout command.  
This is designed to allow control of how Litoka picks a clip without needing to change the URL.  
Note: passing overrides will ignore any filtering that is passed in the initial URL.  

This is performed by including a flag before the target channel's name  
**Example**:  \`!so -r litokabot\`  
#### Overrides:
* Most recent clip: \`-r\` or \`-recent\`
* Most viewed clip: \`-v\` or \`-viewed\` 
* Full Random: \`-f\` or \`-fullrand\`
`;

export default markdown;