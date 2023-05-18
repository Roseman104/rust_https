## Query the Pwned passwords API at Have I been Pwned

Going to start this side to work on querying APIs with rust so I can use it for my alphabet soup project later. 

Lots of chatgpt. Much fun. 

Takes a static input from the main function, sha1 hashes it, asks HIBP if the first 5 of the sha1 are in its API and reports the number of hits in the API 

Using the ["pwned passwords"](https://haveibeenpwned.com/API/v2#PwnedPasswords) functions


Using this project to learn more about Rust programming, http request and security.
### How to Build

Create directory where ever you want as long as rust is installed. 

inside of the directory you want the project to be created:
`cargo new project_name_here`

This will get Cargo to make a project and do some house keeping like make a gitignore so you don't inadvertently try to post executables and so forth to github. 

Copy the contents of the main.rs file to your main and edit the cargo.toml.

Alternatively run:

`cargo add reqwest`

`cargo add sha1`

`cargo add tokio`

`cargo add hex`

I prefer this. 


#### Return

Should return the URL for the page.

Sha1 hash and the clear text

How many times if any it has made it to the Have I been Pwned DB. 