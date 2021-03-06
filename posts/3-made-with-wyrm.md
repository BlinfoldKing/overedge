+++
title: "Made with Wyrm"
subtitle: Yes, I hate writing javascript that much
date:   2021-03-02T17:48:25Z07:00
categories: Software Engineering 
author: Ganesha Danu Enastika
thumbnail: /static/wyrm.png
status: published
+++

# Made with Wyrm
> "F*ck Javascript" - Me, the author

The idea of writing of my own blog is to have to practice my engineering skill without any consequence, as blog is easy to implement but having so much stuff you can play around with, while also train my writing skills. But as a professional procastinator that i am i never made it, and once i do, [the project](https://github.com/BlinfoldKing/ego) were overengineered and hard to maintain. Thus to ~~force~~ motivate myself, i arrange a small competition in [an organization](https://www.linkedin.com/company/helloproclub) that i led, in which the participants have to build their own blog within a time limit. Now that i have to show my goodwill as the giving the challenge, thus i proceed to make my own blog...again.

## The Goals

Learning from my previous mistake, i know that the system should be easy enough to maintain so that i don't get lazy to maintain the project...again. But i also want the project to be *edgy* enough that i don't get bored thinkering with it. 
The goal of this blog system is to built a system that *edgy* enough so that i don't get bored thinkering with it, *simple* enough to maintain, have something to display my art hobby. I also want specifically that the content to be saved on file format. Why you ask? previously as i procastinate to maintain my blog, apparently i forgot to renew my payment information for the server and thus all the content written down on the server were lost. By allowing the content to be saved on a file let me saved it on online git repository with $0 cost.

## Design Decision
First let start with the interface. If you look upon the current home screen, you may think that i looks random, but in reality...it is. But the interface of the post detail were design with a certain purpose, first i want to show off my (mediocore) drawing skill, thus my illustration will take the full screen of the page initally and the user will have to scroll to get the rest of the content. But how do signal the reader to scroll, my solution is to add a gradient that seamlessly transition the content, this was achieve by putting the illustration and the title-content into separate layer, as illustrated below.
![](/static/structure.png)
Now that we get the main idea of the interface out of the way, let's get into the engineering stuff. Let me introduce to you the WYRM + Go stack. W.Y.R.M itself is an acroynm of WASM, Yew, Rust, and Markdown. The idea is to write a front end with the WYR portion of the stack. And yes, no javascript required to build this project. Almost every content of the interface were written in rust using yew library, which will be compiled into your regular html+css+js+wasm code that you can just serve regularly with a web server. In this case i write the web server using go, because other than it was faster than javascript, it is in fact not a javascipt, which something i consider as a plus side. The go web server have two jobs in this project. One of which is to send the previously compiled html+css+js+wasm to the user's browser.
![](/static/process.png)
Now, how about the M of the WYRM. As you may realized that neither the Go or WYR hold the ownership of the blog contents. As i previously stated i want all the posts to be stored as a file data, thus Markdown file format being chosen to do so. The idea is to have a humanly readable format such as markdown to be converted into html and then send to the WYR portion of the app to be rendered. While as of now all the post were written in markdown for simplicity, due to the separation of content and frontend it'll allow the project to render html from various source (react component for example) as long as it could be converted into plain html before sending it to the WYR portion.
![](/static/datasource.png)
Another think that i would like to add is, the comment feature, well technically i don't need to write a single javascript to make this blog complete, i want to add a comment feature so i can get any feedback from any post. But me being a professional procastinator, decided to use disqus as the comment feature of this blog, thus some javascript needed to make it works.

## Future Improvement

* Better home screen
* People said that the light theme hurts their eyes :<
* Content Management System integrated to git

## Related Link
[this project repository...and yes, it's open source](https://github.com/blinfoldking/overedge)  
[Yew](https://yew.rs)  
[Rust](https://www.rust-lang.org/)  
[Disqus](https://disqus.com/)