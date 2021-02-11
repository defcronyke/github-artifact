# GitHub Artifact  
  
[![sponsor the project](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&link=https://github.com/sponsors/defcronyke)](https://github.com/sponsors/defcronyke)  
  
Website: [https://defcronyke.github.io/github-artifact](https://defcronyke.github.io/github-artifact)  
  
Copyright © 2021 [Jeremy Carter](https://eternalvoid.net) - [jeremy@jeremycarter.ca](mailto:jeremy@jeremycarter.ca)  
  
---------------------------  
  
Get the latest build artifact from a GitHub repository:  
-------------------------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo](https://tinyurl.com/github-artifact?repo=user:token@owner/repo)  
  
List all the build artifacts:  
-----------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&num=0](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&num=0)  
  
Get the second most recent build artifact:  
------------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&num=2](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&num=2)  
  
Get the second oldest build artifact:  
-------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&num=-2](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&num=-2)  
  
Get the latest build artifact with a certain name:  
--------------------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name)  
  
List all the build artifacts with a certain name:  
-------------------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name&num=0](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name&num=0)  
  
Get the second most recent build artifact with a certain name:  
--------------------------------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name&num=2](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name&num=2)  
  
Get the second oldest build artifact with a certain name:  
---------------------------------------------------------  
[https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name&num=-2](https://tinyurl.com/github-artifact?repo=user:token@owner/repo&file=name&num=-2)  
  
---------------------------  
  
For example, get the most recent build artifact named "hobnob-release-windows-x86_64" 
from the GitHub repository <a href="https://github.com/defcronyke/hobnob">https://github.com/defcronyke/hobnob</a>:<br>  
  
[https://tinyurl.com/github-artifact?repo=defcronyke:Yjk1MzA1ZTgwMDdmZGIwM2MyMjA1ZGU0MGRkZDNjNjM1OGJjZTFiNQ==@defcronyke/hobnob&file=hobnob-release-windows-x86_64](https://tinyurl.com/github-artifact?repo=defcronyke:Yjk1MzA1ZTgwMDdmZGIwM2MyMjA1ZGU0MGRkZDNjNjM1OGJjZTFiNQ==@defcronyke/hobnob&file=hobnob-release-windows-x86_64)  
  
*NOTE: You can supply your GitHub token as either base64 (as above) or plain text.*  
  
---------------------------  
  
Important  
=========  
These GitHub tokens need the "public_repo" scope to work properly, and unfortunately 
that scope gives read/write access to the application making the GitHub API calls, so 
if you want to use this to make any public links that you'll be sharing anywhere, you 
need to run the Docker container in this project on your own server, and when building 
the container you can supply default `user` and `token` values by setting a docker 
build argument like this: `--build-arg artifact_auth="user:token"`  
  
After setting that, you'll be able to omit the `user:token@` portion of the urls above,
and it will use those values by default. For example:  
  
[https://tinyurl.com/github-artifact?repo=defcronyke/hobnob&file=hobnob-release-windows-x86_64](https://tinyurl.com/github-artifact?repo=defcronyke/hobnob&file=hobnob-release-windows-x86_64)  
  
Take a look at the `build.sh` script for an example of how to build the Docker container,
and see the `Dockerfile` for more details.  
  
Extra Info  
==========  
If you prefer, you can use this website's URL [https://defcronyke.github.io/github-artifact](https://defcronyke.github.io/github-artifact) 
as the base path for your links instead of [https://tinyurl.com/github-artifact](https://tinyurl.com/github-artifact). 
For example:  
  
[https://defcronyke.github.io/github-artifact?repo=defcronyke/hobnob&file=hobnob-release-windows-x86_64](https://defcronyke.github.io/github-artifact?repo=defcronyke/hobnob&file=hobnob-release-windows-x86_64)  
