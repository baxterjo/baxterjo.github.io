
#### ABOUT THE PROJECT

This website is my first foray into building a GUI using Rust. I built a static HTML website (now on an [archive branch](https://github.com/baxterjo/baxterjo.github.io/tree/archive/old-site)) on my way out of college in 2020 and decided that upgrading it to a webapp would be great practice for developing my frontend skills.

I would also really like to drive the point home that **I am not a frontend developer** so this website is in a domain that I am **not** good at.

#### SKILLS USED

|   |  |
|---|---|
|Rust Programming Language| HTML / CSS|
| Dioxus GUI Framework | Linux Operating System |

#### Powered by...

![](/img/software_projects/web_site/dioxus.png)

Dioxus is a framework that allows me to compile this application to WASM from Rust. It is heavily inspired by the very popular [React](https://react.dev/) framework and has many of the same concepts. This framework (and all Rust GUI frameworks for that matter) is still very much in it's adolescent phase, but it is gaining traction fast and has a very active and dedicated core team. Big shout-out to github user [ealmloff](https://github.com/ealmloff) for responding to my posted issues almost always same day and being patient with my newbie frontend skills.

#### Shared Components

One of the greatest challenges of the old site was having common page elements shared between each page. The nav bar at the top of the page for instance, was copy / pasted into each HTML page and had to be edited on every page every time there was a change. A problem of O(n) for a computer is fine, but for a developer it is torture. This website has a few shared components that allow one change to propagate across the entire website.

##### Gallery

The collections of cards that you see on the `Home`, `Experience`, and `Projects` page are dynamically built based on `site_content` (more on that below).

##### NavBar / Footer

Both of these components are written once and propagate everywhere, dynamic styling based on location in the website are built into the components.

#### Adding Content

Another huge challenge on the old site was adding content. I was literally copy / pasting the raw HTML from a different page and editing the body of the page for each 'project' and 'experience 'page in the site. This version has a `site_content` directory at the root of the repository with a specific structure that allows me to add content in markdown and it is baked into the binary. So the next time I want to add a page, I add a folder with the page title, and some config options and its shipped with the next build. I realized about halfway through the project that I had basically recrated [Jekyll](https://jekyllrb.com/).

#### Bootstrap 5

The last major upgrade from the old site is migrating from Bootstrap 3 to Bootstrap 5. Since I am not a frontend developer, figuring out the CSS framework was the lowest velocity portion of the project. The styling of this site is based off of a template from [TemplateMag](https://templatemag.com/) that used bootstrap 3. I had originally considered migrating to [tailwindcss](https://tailwindcss.com/) at the recommendation of the Dioxus team, but decided Bootstrap would work for my needs.