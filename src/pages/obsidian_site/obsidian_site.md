---
date: Tuesday, May 21st 2024
desc: 
tags:
  - spring2024
  - compsci
---
# Scope:
Project to render pages from my obsidian marked a certain way to a webpage,
- this should also render projects referenced on said pages (maybe through github or smth)
- this will run as an application, when users run they can either add files, read currently served files, etc. for each run of the program
# Webpages 
	what they need and stuff
- file path to page being served
- reference to project/repo for the code
	- will likely need to have custom file format for webpages and their associated info
smth like:
```
- /mnt/c/Users/delst/Documents/master/d3/anthill-page
	- /mnt/c/Users/delst/Documents/master/d3/code/rustlab/anthill
```
- this will need a custom location for the project snippet to go...
# Configuring app
### FOR PROJECT PAGES:
- manually add directory for page and project
### FOR NON-PROJECT PAGES
- specify a unique tag/folder for the webpages and serve from said directory
	- USE AUTO-NOTE MOVER ON THE SPECIFIED TAG

# Sync non-project and project pages using same page format 
- when no field is specified for a project, treat it as a non-project page

# what this needs:
Note-Adder
- adds notes/md files to custom parsed file
- remove functionality
- list & view current notes functionality
Note Server
- Breaks down notes into servable parts and serves notes
Project Adder
- adds a project to a specified note, specifying code base or smth to serve js

